use embassy_time::{Duration, Timer};
use firmware::FirmwareInfo;

use crate::{bindings::*, bus::Bus, util::slice32_mut, Error, PBUS, SR1_RPU_AWAKE, SR1_RPU_READY, SR2_RPU_WAKEUP_REQ};

/*
pktram: 0xB0000000 - 0xB0030FFF -- 196kb
usable for mcu-rpu comms: 0xB0005000 - 0xB0030FFF -- 176kb

First we allocate N tx buffers, which consist of
- Header of 52 bytes
- Data of N bytes

Then we allocate rx buffers.
- 3 queues of
  - N buffers each, which consist of
    - Header of 4 bytes
    - Data of N bytes (default 1600)

Each RX buffer has a "descriptor ID" which is assigned across all queues starting from 0
- queue 0 is descriptors 0..N-1
- queue 1 is descriptors N..2N-1
- queue 2 is descriptors 2N..3N-1
*/

// Configurable by user
// const MAX_TX_TOKENS: usize = 10;

const MAX_TX_AGGREGATION: usize = 6;
// const TX_MAX_DATA_SIZE: usize = 1600;
const RX_MAX_DATA_SIZE: u16 = 1600;
const RX_BUFS_PER_QUEUE: u16 = 5;

// Fixed

/*
const TX_BUFS: usize = MAX_TX_TOKENS * MAX_TX_AGGREGATION;
const TX_BUF_SIZE: usize = TX_BUF_HEADROOM as usize + TX_MAX_DATA_SIZE;
const TX_TOTAL_SIZE: usize = TX_BUFS * TX_BUF_SIZE;
*/

const RX_BUFS: usize = (RX_BUFS_PER_QUEUE as usize) * (MAX_NUM_OF_RX_QUEUES as usize);
const RX_BUF_SIZE: usize = RX_BUF_HEADROOM as usize + RX_MAX_DATA_SIZE as usize;
const RX_TOTAL_SIZE: usize = RX_BUFS * RX_BUF_SIZE;

//     // assert!(MAX_TX_TOKENS >= 1, "At least one TX token is required");
//     // assert!(MAX_TX_AGGREGATION <= 16, "Max TX aggregation is 16");
//     // assert!(RX_BUFS_PER_QUEUE >= 1, "At least one RX buffer per queue is required");
//     // assert!(
//     //     (TX_TOTAL_SIZE + RX_TOTAL_SIZE) as u32 <= RPU_PKTRAM_SIZE,
//     //     "Packet RAM overflow"
//     // );

// TODO: should be a config with a range
// const NRF70_RX_NUM_BUFS: u32 = 48;
// const NRF70_RX_MAX_DATA_SIZE: u32 = 1600;

// #define MAX_TX_FRAME_SIZE \
// 	(CONFIG_NRF_WIFI_IFACE_MTU + NRF_WIFI_FMAC_ETH_HDR_LEN + TX_BUF_HEADROOM)
// #define TOTAL_TX_SIZE \
// 	(CONFIG_NRF70_MAX_TX_TOKENS * CONFIG_NRF70_TX_MAX_DATA_SIZE)
// #define TOTAL_RX_SIZE \
// 	(CONFIG_NRF70_RX_NUM_BUFS * CONFIG_NRF70_RX_MAX_DATA_SIZE)

// config NRF70_RX_NUM_BUFS
// 	int "Number of RX buffers"
// 	default 48
//
// config NRF70_MAX_TX_AGGREGATION
// 	int "Maximum number of TX packets to aggregate"
// 	default 12
//
// config NRF70_MAX_TX_TOKENS
// 	int "Maximum number of TX tokens"
// 	range 5 12 if !NRF70_RADIO_TEST
// 	default 10
//
// config NRF70_TX_MAX_DATA_SIZE
// 	int "Maximum size of TX data"
// 	default 1600
//
// config NRF70_RX_MAX_DATA_SIZE
// 	int "Maximum size of RX data"
// 	default 1600

pub(crate) mod commands;
pub(crate) mod firmware;
pub(crate) mod memory;
pub(crate) mod rf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum ProcessorType {
    Lmac,
    Umac,
}

pub struct Rpu<BUS: Bus> {
    bus: BUS,

    hostport_queues_info: Option<host_rpu_hpqm_info>,

    rx_command_base_address: Option<u32>,
    tx_command_base_address: Option<u32>,

    num_commands: u32,
}

#[allow(dead_code)]
impl<BUS: Bus> Rpu<BUS> {
    pub fn new(bus: BUS) -> Self {
        Rpu {
            bus,

            hostport_queues_info: None,

            rx_command_base_address: None,
            tx_command_base_address: None,

            num_commands: RPU_CMD_START_MAGIC,
        }
    }

    pub async fn boot<'firmware_info_lifetime>(
        &mut self,
        firmware_info: &FirmwareInfo<'firmware_info_lifetime>,
    ) -> Result<(), Error> {
        self.wake_up().await?;

        self.enable_clocks().await;
        self.enable_interrupts().await;

        self.reset().await;

        self.firmware_load(firmware_info).await;
        self.firmware_boot().await;

        let version = self.firmware_version().await;

        info!(
            "Firmware for RPU ({}.{}.{}.{}) booted sucessfully",
            version.version, version.major, version.minor, version.extra
        );

        // Done in Zephyr sample FW, maybe not necessary
        self.wake_up().await?;

        // -- Retrieve HPQM information ---

        // Read the host port queue info for all the queues provided by the RPU (like command, event, RX buffer queues etc)
        let mut hpqm_info_buffer = [0; size_of::<host_rpu_hpqm_info>()];
        self.read_buffer(RPU_MEM_HPQ_INFO, None, slice32_mut(&mut hpqm_info_buffer))
            .await;

        self.hostport_queues_info = Some(unsafe { core::mem::transmute_copy(&hpqm_info_buffer) });

        // Fetch the addresses for the RX and TX command bases
        self.rx_command_base_address = Some(self.read_u32(RPU_MEM_RX_CMD_BASE, None).await);
        self.tx_command_base_address = Some(RPU_MEM_TX_CMD_BASE);

        // -- Retrieve OTP info ---
        let mut otp_info_buffer = [0; size_of::<host_rpu_umac_info>()];
        self.read_buffer(RPU_MEM_UMAC_BOOT_SIG, None, slice32_mut(&mut otp_info_buffer))
            .await;

        let otp_info = unsafe { core::mem::transmute_copy(&otp_info_buffer) };
        let otp_flags = self.read_u32(RPU_MEM_OTP_INFO_FLAGS, None).await;

        // -- Retrieve RF parameters ---

        // DTS uses 1dBm as the unit for TX power, while the RPU uses 0.25dBm, so multiply by 4
        let tx_pwr_ceil_params = nrf_wifi_tx_pwr_ceil_params {
            max_pwr_2g_dsss: 21 * 4,
            max_pwr_2g_mcs0: 16 * 4,
            max_pwr_2g_mcs7: 16 * 4,
            max_pwr_5g_low_mcs0: 9 * 4,
            max_pwr_5g_low_mcs7: 9 * 4,
            max_pwr_5g_mid_mcs0: 11 * 4,
            max_pwr_5g_mid_mcs7: 11 * 4,
            max_pwr_5g_high_mcs0: 13 * 4,
            max_pwr_5g_high_mcs7: 13 * 4,
        };

        let rf_parameters = self.get_rf_parameters(&otp_info, otp_flags, &tx_pwr_ceil_params).await;

        // -- Initialize RX queues ---

        for queue_index in 0..(MAX_NUM_OF_RX_QUEUES as usize) {
            for buffer_index in 0..RX_BUFS_PER_QUEUE {
                let descriptor_identifier = queue_index * (RX_BUFS_PER_QUEUE as usize) + buffer_index as usize;

                // TODO: Is this correct?
                let rpu_addr = (RPU_MEM_PKT_BASE + RPU_PKTRAM_SIZE - RX_TOTAL_SIZE as u32)
                    + (RX_BUF_SIZE * descriptor_identifier) as u32;

                // Write RX buffer header
                self.write_u32(rpu_addr, None, descriptor_identifier as u32).await;

                // Create host_rpu_rx_buf_info (it's just one word of the address)
                let command = [rpu_addr + RX_BUF_HEADROOM];

                // Call wifi_nrf_hal_data_cmd_send with the command
                self.send_rx_command(&command, descriptor_identifier as u32, queue_index)
                    .await?;
            }
        }

        // --- Initialize the firmware ---

        self.firmware_initialize(&rf_parameters).await?;

        // Wait for event init done

        // TODO: after init, set MAC address
        // --- Check MAC address ---

        let mac_address = [
            (otp_info.mac_address0[0]) as u8,
            (otp_info.mac_address0[0] >> 8) as u8,
            (otp_info.mac_address0[0] >> 16) as u8,
            0x0,
            (otp_info.mac_address0[1]) as u8,
            (otp_info.mac_address0[1] >> 8) as u8,
        ];

        info!(
            "MAC address: {:#x}:{:#x}:{:#x}:{:#x}:{:#x}:{:#x}",
            mac_address[0], mac_address[1], mac_address[2], mac_address[3], mac_address[4], mac_address[5]
        );

        // TODO: Send command

        // TODO: bring interface up: nrf_wifi_fmac_chg_vif_state and wait for event

        // let result = self.read_u32_from_region(SYSBUS, 0x0C0).await;
        // info!("PART: {}", result);

        Ok(())
    }

    pub async fn wait_for_event(
        &mut self,
        buffer: &mut [u32; (MAX_EVENT_POOL_LEN / 4) as usize],
    ) -> Result<host_rpu_msg, Error> {
        let hostport_queues_info = match self.hostport_queues_info {
            Some(hostport_queues_info) => Ok(hostport_queues_info),
            None => Err(Error::NotInitialized),
        }?;

        loop {
            let event_address = self.hostport_queue_dequeue(hostport_queues_info.event_busy_queue).await;

            let event_address = match event_address {
                // No more events to read. Sometimes when low power mode is enabled
                // we see a wrong address, but it work after a while, so, add a
                // check for that.
                None | Some(0xAAAA_AAAA) => {
                    Timer::after_millis(100).await;
                    continue;
                }
                Some(event_address) => event_address,
            };

            debug!("Got event. Address: {:#x}", event_address);

            return self.read_event(event_address, buffer).await;
        }
    }

    pub async fn irq_ack(&mut self) {
        // TODO: I think this clears the interrupt flag
        self.write_u32(RPU_REG_INT_FROM_MCU_ACK, None, 1 << RPU_REG_BIT_INT_FROM_MCU_ACK)
            .await;
    }

    /// Checks if the watchdog was the source of the interrupt
    pub async fn irq_watchdog_check(&mut self) -> bool {
        let val = self.read_u32(RPU_REG_MIPS_MCU_UCCP_INT_STATUS, None).await;
        (val & (1 << RPU_REG_BIT_MIPS_WATCHDOG_INT_STATUS)) > 0
    }

    pub async fn irq_watchdog_ack(&mut self) {
        self.write_u32(
            RPU_REG_MIPS_MCU_UCCP_INT_CLEAR,
            None,
            1 << RPU_REG_BIT_MIPS_WATCHDOG_INT_CLEAR,
        )
        .await;
    }
}

#[allow(dead_code)]
impl<BUS: Bus> Rpu<BUS> {
    async fn wake_up(&mut self) -> Result<(), Error> {
        debug!("Waking up...");

        self.bus.write_sr2(SR2_RPU_WAKEUP_REQ).await;

        self.wait_for_wakeup_request_ack().await?;

        self.wait_until_awake().await
    }

    async fn sleep(&mut self) {
        debug!("Sleeping...");

        self.bus.write_sr2(0).await;
    }

    async fn reset(&mut self) {
        let processors = [ProcessorType::Lmac, ProcessorType::Umac];

        for processor in processors {
            let control_register_address = match processor {
                ProcessorType::Lmac => RPU_REG_MIPS_MCU_CONTROL,
                ProcessorType::Umac => RPU_REG_MIPS_MCU2_CONTROL,
            };

            // Do pulsed soft reset
            self.write_u32(control_register_address, Some(processor), 0x1).await;

            // Wait for it to come out of reset
            while self.read_u32(control_register_address, Some(processor)).await & 0x1 != 0 {}

            // MIPS will restart from its boot exception registers and hit its default wait instruction
            let boot_exception_register_address = match processor {
                ProcessorType::Lmac => 0xA400_0018,
                ProcessorType::Umac => 0xA400_0118,
            };

            while self.read_u32(boot_exception_register_address, Some(processor)).await & 0x01 != 1 {}
        }
    }

    async fn enable_clocks(&mut self) {
        debug!("Enabling clocks...");
        self.write_u32_to_region(PBUS, 0x8C20, 0x0100).await;
    }

    async fn enable_interrupts(&mut self) {
        debug!("Enabling interrupts...");

        // First enable the block-wise interrupt for the relevant block in the master register
        let mut value = self.read_u32(RPU_REG_INT_FROM_RPU_CTRL, None).await;

        value |= 1 << RPU_REG_BIT_INT_FROM_RPU_CTRL;

        self.write_u32(RPU_REG_INT_FROM_RPU_CTRL, None, value).await;

        // Now enable the relevant MCU interrupt line
        self.write_u32(RPU_REG_INT_FROM_MCU_CTRL, None, 1 << RPU_REG_BIT_INT_FROM_MCU_CTRL)
            .await;
    }

    async fn disable_interrupts(&mut self) {
        debug!("Disabling interrupts...");

        let mut value = self.read_u32(RPU_REG_INT_FROM_RPU_CTRL, None).await;
        value &= !(1 << RPU_REG_BIT_INT_FROM_RPU_CTRL);

        self.write_u32(RPU_REG_INT_FROM_RPU_CTRL, None, value).await;

        self.write_u32(RPU_REG_INT_FROM_MCU_CTRL, None, !(1 << RPU_REG_BIT_INT_FROM_MCU_CTRL))
            .await;
    }

    async fn write_core(&mut self, core_address: u32, buf: &[u32], processor: ProcessorType) {
        // We receive the address as a byte address, while we need to write it as a word address
        let addr = (core_address & RPU_ADDR_MASK_OFFSET) / 4;

        let (addr_reg, data_reg) = match processor {
            ProcessorType::Lmac => (RPU_REG_MIPS_MCU_SYS_CORE_MEM_CTRL, RPU_REG_MIPS_MCU_SYS_CORE_MEM_WDATA),
            ProcessorType::Umac => (
                RPU_REG_MIPS_MCU2_SYS_CORE_MEM_CTRL,
                RPU_REG_MIPS_MCU2_SYS_CORE_MEM_WDATA,
            ),
        };

        // Write the processor address register
        self.write_u32(addr_reg, Some(processor), addr).await;

        // Write to the data register one by one
        for data in buf {
            self.write_u32(data_reg, Some(processor), *data).await;
        }
    }

    async fn read_event(&mut self, event_address: u32, buf: &mut [u32]) -> Result<host_rpu_msg, Error> {
        const MESSAGE_WITHOUT_DATA_SIZE: usize = core::mem::size_of::<host_rpu_msg>();
        let mut message_buffer = [0; MESSAGE_WITHOUT_DATA_SIZE];

        self.read_buffer(event_address, None, slice32_mut(&mut message_buffer))
            .await;

        let message: host_rpu_msg = unsafe { core::mem::transmute_copy(&message_buffer) };

        self.read_buffer(
            event_address + MESSAGE_WITHOUT_DATA_SIZE as u32,
            None,
            &mut buf[..RPU_EVENT_COMMON_SIZE_MAX as usize / 4],
        )
        .await;

        // Get the header from the front of the event data
        if message.hdr.resubmit > 0 {
            self.free_event(event_address).await?;
        }

        let len = message.hdr.len as usize;
        if len > MAX_EVENT_POOL_LEN as usize {
            todo!("Fragmented event read is not yet implemented");
        } else if len > RPU_EVENT_COMMON_SIZE_MAX as usize {
            // This is a longer than usual event. We gotta read it again
            self.read_buffer(
                event_address + MESSAGE_WITHOUT_DATA_SIZE as u32,
                None,
                &mut buf[..(len + 3) / 4],
            )
            .await;
        }

        Ok(message)
    }

    async fn free_event(&mut self, event_address: u32) -> Result<(), Error> {
        let hostport_queues_info = match self.hostport_queues_info {
            Some(hostport_queues_info) => Ok(hostport_queues_info),
            None => Err(Error::NotInitialized),
        }?;

        self.hostport_queue_enqueue(hostport_queues_info.event_avl_queue, event_address)
            .await;

        Ok(())
    }

    async fn wait_for_wakeup_request_ack(&mut self) -> Result<(), Error> {
        for _ in 0..10 {
            if self.bus.read_sr2().await == SR2_RPU_WAKEUP_REQ {
                return Ok(());
            }
            Timer::after(Duration::from_millis(1)).await;
        }

        Err(Error::NoAcknowledgement)
    }

    async fn wait_until_awake(&mut self) -> Result<(), Error> {
        for _ in 0..10 {
            if self.bus.read_sr1().await & SR1_RPU_AWAKE != 0 {
                return Ok(());
            }
            Timer::after(Duration::from_millis(1)).await;
        }

        Err(Error::Timeout)
    }

    async fn wait_until_ready(&mut self) -> Result<(), Error> {
        for _ in 0..10 {
            if self.bus.read_sr1().await == SR1_RPU_AWAKE | SR1_RPU_READY {
                return Ok(());
            }
            Timer::after(Duration::from_millis(1)).await;
        }

        Err(Error::Timeout)
    }

    async fn get_sleep_status(&mut self) -> u8 {
        self.bus.read_sr1().await
    }

    async fn hostport_queue_enqueue(&mut self, hostport_queue: host_rpu_hpq, value: u32) {
        // TODO: temp
        // match self.wake_up().await {
        //     Ok(()) => (),
        //     Err(error) => error!("Failed to wake up: {:?}", error),
        // }
        self.write_u32(hostport_queue.enqueue_addr, None, value).await;
    }

    async fn hostport_queue_dequeue(&mut self, hostport_queue: host_rpu_hpq) -> Option<u32> {
        let value = self.read_u32(hostport_queue.dequeue_addr, None).await;

        // Pop element only if it is valid
        if value != 0 {
            self.write_u32(hostport_queue.dequeue_addr, None, value).await;
            Some(value)
        } else {
            None
        }
    }
}

impl ft_prog_ver {
    fn from_u32(value: u32) -> Option<ft_prog_ver> {
        match value {
            1 => Some(ft_prog_ver::FT_PROG_VER1),
            2 => Some(ft_prog_ver::FT_PROG_VER2),
            3 => Some(ft_prog_ver::FT_PROG_VER3),
            _ => None,
        }
    }
}
