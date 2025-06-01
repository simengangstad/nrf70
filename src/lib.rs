#![no_std]
#![deny(unused_must_use)]
#![allow(async_fn_in_trait)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

pub(crate) mod fmt;

use action::{Action, ActionState};
use bindings::*;
use bus::Bus;
use embassy_futures::select::{select3, Either3};
use embassy_net_driver_channel as ch;
use embassy_time::{Duration, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use fmt::Bytes;
use rpu::firmware::{FirmwareInfo, FirmwareParseError};
use rpu::memory::regions::*;
use rpu::Rpu;
use util::{meh, slice8, unsliceit};

mod action;
pub mod bus;
pub mod control;
mod rpu;
mod util;

#[allow(dead_code)]
mod bindings;

const MTU: usize = 1514;

// const SR0_WRITE_IN_PROGRESS: u8 = 0x01;
const SR1_RPU_AWAKE: u8 = 0x02;
const SR1_RPU_READY: u8 = 0x04;
const SR2_RPU_WAKEUP_REQ: u8 = 0x01;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    NoAcknowledgement,
    Timeout,
    InvalidAddress,
    NotInitialized,
    BufferTooSmall,
    Busy,
    FirmwareParseError(FirmwareParseError),
}

pub struct State {
    action_state: ActionState,
    ch: ch::State<MTU, 4, 4>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ch: ch::State::new(),
            action_state: ActionState::new(),
        }
    }
}

#[allow(dead_code)]
pub struct Control<'a> {
    action_state: &'a ActionState,
    state_ch: ch::StateRunner<'a>,
}

pub type NetDriver<'a> = ch::Device<'a, MTU>;

#[allow(dead_code)]
pub struct Runner<'a, BUS: Bus, IN: InputPin + Wait, OUT: OutputPin> {
    ch: ch::Runner<'a, MTU>,
    state_ch: ch::StateRunner<'a>,
    action_state: &'a ActionState,

    rpu: Rpu<BUS>,
    bucken: OUT,
    iovdd_ctl: OUT,
    host_irq: IN,
}

pub async fn new<'a, BUS, IN, OUT>(
    state: &'a mut State,
    bus: BUS,
    bucken: OUT,
    iovdd_ctl: OUT,
    host_irq: IN,
) -> (NetDriver<'a>, Control<'a>, Runner<'a, BUS, IN, OUT>)
where
    BUS: Bus,
    IN: InputPin + Wait,
    OUT: OutputPin,
{
    let (ch_runner, device) = ch::new(&mut state.ch, ch::driver::HardwareAddress::Ethernet([0; 6]));
    let state_ch = ch_runner.state_runner();

    let mut runner = Runner {
        ch: ch_runner,
        state_ch,
        action_state: &state.action_state,
        rpu: Rpu::new(bus),
        bucken,
        iovdd_ctl,
        host_irq,
    };
    runner.init().await;

    let control = Control {
        action_state: &state.action_state,
        state_ch,
    };

    (device, control, runner)
}

impl<'a, BUS: Bus, IN: InputPin + Wait, OUT: OutputPin> Runner<'a, BUS, IN, OUT> {
    async fn init(&mut self) {
        Timer::after(Duration::from_millis(10)).await;
        self.bucken.set_high().unwrap();
        Timer::after(Duration::from_millis(10)).await;
        self.iovdd_ctl.set_high().unwrap();
        Timer::after(Duration::from_millis(10)).await;
    }

    pub async fn run(&mut self) -> ! {
        let mut buffer = [0u32; (MAX_EVENT_POOL_LEN / 4) as usize];

        // Outer loop waits for IRQ or control
        loop {
            // match select(
            //     async {
            //         self.host_irq.wait_for_high().await;
            //         // *AND* the buffer is ready...
            //         // self.rx_chan.rx_buf().await
            //     },
            //     // ... or a TX buffer becoming available, i.e. embassy-net wants to send a packet
            //     // self.tx_chan.tx_buf(),
            // )
            // .await
            // {
            //     Either::Left(buf) => {
            //         self.rpu.irq_ack().await;
            //         // a packet is ready to be received!
            //         // let n = receive_packet_over_spi(buf).await;
            //         // rx_chan.rx_done(n);
            //     }
            //     Either::Right(_) => {
            //         // a packet is ready to be sent!
            //         // send_packet_over_spi(buf).await;
            //         // tx_chan.tx_done();
            //     }
            // }

            let action = self.action_state.wait_pending();
            let wifi_tx = self.ch.tx_buf();
            let irq_event = self.host_irq.wait_for_high();

            // Need select here for control
            //
            // Send command in case of control
            //
            // Wait for TX buffer from ch (on runner). This is the net layer
            // This is basically the entrypoint for sending packets

            match select3(action, wifi_tx, irq_event).await {
                Either3::First(action) => {
                    debug!("Action: {:?}", action);

                    match action {
                        Action::LoadFirmware(firmware) => match self.load_firmware(firmware).await {
                            Ok(()) => (),
                            Err(error) => self.action_state.respond(Err(error)),
                        },
                        Action::Command((kind, wait_for_completion, buffer, _)) => {
                            match self.rpu.send_command_raw(kind, buffer).await {
                                Ok(()) => {
                                    if !wait_for_completion {
                                        self.action_state.respond(Ok(None));
                                    }
                                }
                                Err(error) => self.action_state.respond(Err(error)),
                            }
                        }
                    };
                }
                Either3::Second(packet) => {
                    debug!("tx pkt {:02x}", Bytes(&packet[..packet.len().min(48)]));
                }
                Either3::Third(irq) => {
                    debug!("Got IRQ");

                    match irq {
                        Ok(()) => {
                            self.rpu.irq_ack().await;
                        }
                        Err(_) => continue,
                    }

                    let event = self.rpu.wait_for_event(&mut buffer).await;

                    if let Ok(message) = event {
                        let buf = slice8(&buffer);

                        if let Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM) =
                            nrf_wifi_host_rpu_msg_type::try_from(message.type_ as u32)
                        {
                            let sys_head: &nrf_wifi_sys_head = unsliceit(buf);

                            let event = nrf_wifi_sys_events::try_from(sys_head.cmd_event as u32);

                            if let Ok(event) = event {
                                info!("Got event: {:?}", event);
                            }

                            match event {
                                Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_INIT_DONE) => {
                                    self.action_state.respond(Ok(None));
                                }
                                Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_STATS) => {
                                    self.action_state.respond(Ok(Some(&buf[..(message.hdr.len as usize)])))
                                }
                                _ => warn!("Event not handled: {:08x}", meh(sys_head.cmd_event)),
                            }
                        } else if let Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC) =
                            nrf_wifi_host_rpu_msg_type::try_from(message.type_ as u32)
                        {
                            let umac_head: &nrf_wifi_umac_head = unsliceit(buf);

                            let cmd = umac_head.cmd;
                            let len = umac_head.len;
                            info!("UMAC command: {} {}", cmd, len);

                            // let event = nrf_wifi_umac_data_commands::try_from(umac_head.cmd as u32);
                            //
                            // if let Ok(event) = event {
                            //     info!("Got event: {:?}", event);
                            // }
                            //
                            // match event {
                            //     Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_INIT_DONE) => {
                            //         self.action_state.respond(Ok(None));
                            //     }
                            //     Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_STATS) => {
                            //         self.action_state.respond(Ok(Some(&buf[..(message.hdr.len as usize)])))
                            //     }
                            //     _ => warn!("Event not handled: {:08x}", meh(umac_head.cmd_event)),
                            // }
                        } else {
                            warn!("unknown event type {:08x}", meh(message.type_));
                        }
                    }

                    if self.rpu.irq_watchdog_check().await {
                        self.rpu.irq_watchdog_ack().await;
                    }
                }
            }
        }
    }

    async fn load_firmware(&mut self, firmware: *const [u8]) -> Result<(), Error> {
        let firmware_info = FirmwareInfo::read(firmware)?;

        self.rpu.boot(&firmware_info).await?;

        Ok(())
    }
}
