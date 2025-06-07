#![no_std]
#![deny(unused_must_use)]
#![allow(async_fn_in_trait)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

pub(crate) mod fmt;

use core::mem::transmute;

use action::{Action, ActionState, Item};
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
use util::{meh, slice8, sliceit, unsliceit, unsliceit2};

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
    NoData,
    Busy,
    FirmwareParseError(FirmwareParseError),
    Code(i32),
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
                        Action::Boot(firmware) => match self.boot(firmware).await {
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
                        Action::Get((item, _)) => match item {
                            Item::UmacInfo => {
                                let umac_info = self.rpu.retrieve_umac_info().await;
                                let umac_info_buffer = sliceit(&umac_info);

                                self.action_state.respond(Ok(Some(&umac_info_buffer[..])));
                            }
                        },
                    };
                }
                Either3::Second(packet) => {
                    debug!("tx pkt {:02x}", Bytes(&packet[..packet.len().min(48)]));
                }
                Either3::Third(irq) => {
                    debug!("Got IRQ, checking event queue...");

                    match irq {
                        Ok(()) => {
                            self.rpu.irq_ack().await;
                        }
                        Err(_) => continue,
                    }

                    let event = self.rpu.read_event(&mut buffer).await;

                    if let Ok(message) = event {
                        let message_type = message.type_ as u32;
                        let message_length = message.hdr.len as usize;

                        let message_type = match nrf_wifi_host_rpu_msg_type::try_from(message_type) {
                            Ok(message_type) => {
                                debug!("Got {:?} event. Message length: {}", message_type, message_length);
                                Some(message_type)
                            }
                            Err(_) => {
                                warn!("Unknown event type {:08x}", message_type);
                                None
                            }
                        };

                        let buf = slice8(&buffer);

                        if let Some(message_type) = message_type {
                            match message_type {
                                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM => {
                                    let header: &nrf_wifi_sys_head = unsliceit(buf);
                                    let event = nrf_wifi_sys_events::try_from(header.cmd_event as u32);
                                    let payload_length = header.len;

                                    if let Ok(event) = event {
                                        debug!("Processing system event: {:?}, length: {}", event, payload_length);
                                    }

                                    match event {
                                        Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_INIT_DONE) => {
                                            self.action_state.respond(Ok(None));
                                        }
                                        Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_STATS) => {
                                            self.action_state.respond(Ok(Some(&buf[..(message.hdr.len as usize)])))
                                        }
                                        _ => warn!("System event not handled: {:08x}", meh(header.cmd_event)),
                                    }
                                }
                                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC => {
                                    let header: &nrf_wifi_umac_hdr = unsliceit(buf);
                                    let event = nrf_wifi_umac_events::try_from(header.cmd_evnt as u32);
                                    let header_length = size_of::<nrf_wifi_umac_hdr>();
                                    let payload_length = message_length - header_length;

                                    if let Ok(event) = event {
                                        debug!("Processing UMAC event: {:?}, length: {}", event, payload_length);
                                    }

                                    match event {
                                        Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CMD_STATUS) => {
                                            let command: &nrf_wifi_umac_event_cmd_status = unsliceit(buf);
                                            let status = command.cmd_status;

                                            let command_type = nrf_wifi_umac_commands::try_from(meh(command.cmd_id));

                                            if let Ok(command_type) = command_type {
                                                debug!(
                                                    "Command {:?} ({:#x}) finished with status {}",
                                                    command_type,
                                                    meh(command.cmd_id),
                                                    status
                                                );
                                            } else {
                                                debug!(
                                                    "Command UNKNOWN ({:#x}) finished with status {}",
                                                    meh(command.cmd_id),
                                                    status
                                                );
                                            }

                                            match status {
                                                0 => self.action_state.respond(Ok(None)),
                                                error => self.action_state.respond(Err(Error::Code(error as i32))),
                                            }
                                        }
                                        Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_IFFLAGS_STATUS) => {
                                            let state: &nrf_wifi_umac_event_vif_state = unsliceit(buf);
                                            let status = state.status;

                                            debug!("Interface flags update finished with status {}", status);

                                            match status {
                                                0 => self.action_state.respond(Ok(None)),
                                                error => self.action_state.respond(Err(Error::Code(error))),
                                            }
                                        }
                                        Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TRIGGER_SCAN_START) => {
                                            let response: &nrf_wifi_umac_event_trigger_scan = unsliceit(buf);

                                            debug!(
                                                "Scan started. Flags {:#x}. SSIDs: {}. Frequencies: {}.",
                                                meh(response.nrf_wifi_scan_flags),
                                                meh(response.num_scan_ssid),
                                                meh(response.num_scan_frequencies)
                                            );
                                        }
                                        _ => warn!("UMAC event not handled: {:#08x}", meh(header.cmd_evnt)),
                                    }

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
                                }
                                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_DATA => {
                                    let header: &nrf_wifi_umac_head = unsliceit(buf);
                                    let command = nrf_wifi_umac_data_commands::try_from(header.cmd);

                                    if let Ok(command) = command {
                                        debug!("Processing DATA command: {:?}, length: {}", command, meh(header.len));
                                    }

                                    match command {
                                        Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_ON) => {
                                            let carrier_state: &nrf_wifi_data_carrier_state = unsliceit(buf);
                                            info!("Carrier state ON for WDEV {}", meh(carrier_state.wdev_id));
                                        }
                                        Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_OFF) => {
                                            let carrier_state: &nrf_wifi_data_carrier_state = unsliceit(buf);
                                            info!("Carrier state OFF for WDEV {}", meh(carrier_state.wdev_id));
                                        }
                                        Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_RX_BUFF) => {
                                            let (rx_packet, buf) = unsliceit2::<nrf_wifi_rx_buff>(buf);

                                            let rx_packet_type =
                                                nrf_wifi_rx_pkt_type::try_from(meh(rx_packet.rx_pkt_type) as u32);

                                            let number_of_packets = rx_packet.rx_pkt_cnt as usize;

                                            debug!(
                                                "Got RX buffer. # packets: {}. Frequency: {}",
                                                number_of_packets,
                                                meh(rx_packet.frequency)
                                            );

                                            match rx_packet_type {
                                                Ok(nrf_wifi_rx_pkt_type::NRF_WIFI_RX_PKT_BCN_PRB_RSP) => {
                                                    // Create slice of rx_buffer_info

                                                    let rx_buffer_infos: &[nrf_wifi_rx_buff_info] =
                                                        unsafe { transmute(buf) };

                                                    for packet_index in 0..number_of_packets {
                                                        // Go through
                                                        let rx_buffer_info = rx_buffer_infos[packet_index];

                                                        let packet_descriptor = rx_buffer_info.descriptor_id;
                                                        let packet_length = rx_buffer_info.rx_pkt_len;
                                                        let packet_type = rx_buffer_info.pkt_type;

                                                        debug!(
                                                            "RX packet - Descriptor: {}. Length: {}. Type: {}",
                                                            packet_descriptor, packet_length, packet_type
                                                        );

                                                        // Do we need to read out then?
                                                    }
                                                }
                                                _ => {
                                                    warn!("Unknown RX packet type: {:#x}", meh(rx_packet.rx_pkt_type))
                                                }
                                            }
                                        }
                                        _ => warn!("DATA command not handled: {:08x}", meh(header.cmd)),
                                    }
                                }
                                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SUPPLICANT => {
                                    debug!("Got supplicant event, ignoring...");
                                }
                            }
                        }
                    }

                    if self.rpu.irq_watchdog_check().await {
                        self.rpu.irq_watchdog_ack().await;
                    }
                }
            }
        }
    }

    async fn boot(&mut self, firmware: *const [u8]) -> Result<(), Error> {
        let firmware_info = FirmwareInfo::read(firmware)?;
        self.rpu.boot(&firmware_info).await
    }
}
