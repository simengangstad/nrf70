use core::mem::{size_of, zeroed};

use embassy_time::{Duration, Timer};

use crate::{
    bindings::{
        host_rpu_msg, host_rpu_msg_hdr, nrf_wifi_cmd_get_stats, nrf_wifi_cmd_get_wiphy, nrf_wifi_cmd_sys_deinit,
        nrf_wifi_cmd_sys_init, nrf_wifi_host_rpu_msg_type, nrf_wifi_ie, nrf_wifi_index_ids, nrf_wifi_scan_params,
        nrf_wifi_sys_commands, nrf_wifi_sys_head, nrf_wifi_umac_chg_vif_state_info, nrf_wifi_umac_cmd_abort_scan,
        nrf_wifi_umac_cmd_add_vif, nrf_wifi_umac_cmd_change_macaddr, nrf_wifi_umac_cmd_chg_sta,
        nrf_wifi_umac_cmd_chg_vif_state, nrf_wifi_umac_cmd_get_scan_results, nrf_wifi_umac_cmd_key,
        nrf_wifi_umac_cmd_mcast_filter, nrf_wifi_umac_cmd_mgmt_frame_reg, nrf_wifi_umac_cmd_scan,
        nrf_wifi_umac_cmd_set_power_save, nrf_wifi_umac_commands, nrf_wifi_umac_hdr, nrf_wifi_umac_scan_info,
        rpu_stats_type, scan_reason, MAX_NRF_WIFI_UMAC_CMD_SIZE, NRF_WIFI_HAL_MSG_TYPE,
        NRF_WIFI_INDEX_IDS_WDEV_ID_VALID, RPU_ADDR_MASK_OFFSET, RPU_DATA_CMD_SIZE_MAX_RX, RPU_MCU_CORE_INDIRECT_BASE,
        RPU_REG_INT_TO_MCU_CTRL,
    },
    bus::Bus,
    rpu::{Error, ProcessorType},
    slice8,
    util::{slice32, slice8_mut, sliceit},
};

use super::Rpu;

pub trait Command {
    const MESSAGE_TYPE: nrf_wifi_host_rpu_msg_type;
    fn prepare(&mut self);

    fn domain(&self) -> nrf_wifi_host_rpu_msg_type {
        return Self::MESSAGE_TYPE;
    }

    fn kind(&self) -> u32;
}

macro_rules! impl_cmd {
    (sys, $cmd:path, $num:expr) => {
        impl Command for $cmd {
            const MESSAGE_TYPE: nrf_wifi_host_rpu_msg_type =
                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM;
            fn prepare(&mut self) {
                self.sys_head.cmd_event = $num as _;
                self.sys_head.len = size_of::<Self>() as _;
            }

            fn kind(&self) -> u32 {
                self.sys_head.cmd_event
            }
        }
    };
    (umac, $cmd:path, $num:expr) => {
        impl Command for $cmd {
            const MESSAGE_TYPE: nrf_wifi_host_rpu_msg_type =
                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC;
            fn prepare(&mut self) {
                self.umac_hdr.cmd_evnt = $num as _;
            }

            fn kind(&self) -> u32 {
                self.umac_hdr.cmd_evnt
            }
        }
    };
}

impl Default for nrf_wifi_umac_hdr {
    fn default() -> Self {
        nrf_wifi_umac_hdr {
            cmd_evnt: 0,
            ids: nrf_wifi_index_ids {
                valid_fields: NRF_WIFI_INDEX_IDS_WDEV_ID_VALID,
                ifaceindex: 0,
                nrf_wifi_wiphy_idx: 0,
                wdev_id: 0,
            },
            portid: 0,
            seq: 0,
            rpu_ret_val: 0,
        }
    }
}

impl_cmd!(sys, nrf_wifi_cmd_sys_init, nrf_wifi_sys_commands::NRF_WIFI_CMD_INIT);

impl_cmd!(
    sys,
    nrf_wifi_cmd_get_stats,
    nrf_wifi_sys_commands::NRF_WIFI_CMD_GET_STATS
);

impl Default for nrf_wifi_cmd_get_stats {
    fn default() -> Self {
        let mut cmd = nrf_wifi_cmd_get_stats {
            sys_head: nrf_wifi_sys_head { cmd_event: 0, len: 1 },
            stats_type: rpu_stats_type::RPU_STATS_TYPE_ALL as i32,
            op_mode: 0,
        };
        cmd.prepare();
        cmd
    }
}

impl_cmd!(sys, nrf_wifi_cmd_sys_deinit, nrf_wifi_sys_commands::NRF_WIFI_CMD_DEINIT);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_add_vif,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_INTERFACE
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_change_macaddr,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CHANGE_MACADDR
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_chg_vif_state,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_IFFLAGS
);

impl Default for nrf_wifi_umac_cmd_chg_vif_state {
    fn default() -> Self {
        let mut cmd = nrf_wifi_umac_cmd_chg_vif_state {
            umac_hdr: nrf_wifi_umac_hdr::default(),
            info: nrf_wifi_umac_chg_vif_state_info { state: 0, if_index: 0 },
        };
        cmd.prepare();
        cmd
    }
}

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_scan,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_TRIGGER_SCAN
);

impl Default for nrf_wifi_umac_cmd_scan {
    fn default() -> Self {
        let mut cmd = nrf_wifi_umac_cmd_scan {
            umac_hdr: nrf_wifi_umac_hdr::default(),
            info: nrf_wifi_umac_scan_info {
                scan_reason: scan_reason::SCAN_DISPLAY as i32,
                scan_params: nrf_wifi_scan_params {
                    passive_scan: 0,
                    num_scan_ssids: 0,
                    scan_ssids: unsafe { zeroed() },
                    no_cck: 0,
                    bands: 0,
                    ie: nrf_wifi_ie {
                        ie_len: 0,
                        ie: unsafe { zeroed() },
                    },
                    mac_addr: unsafe { zeroed() },
                    dwell_time_active: 0,
                    dwell_time_passive: 0,
                    num_scan_channels: 0,
                    skip_local_admin_macs: 0,
                    center_frequency: unsafe { zeroed() },
                },
            },
        };
        cmd.prepare();
        cmd
    }
}

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_abort_scan,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_ABORT_SCAN
);

impl Default for nrf_wifi_umac_cmd_abort_scan {
    fn default() -> Self {
        let mut cmd = nrf_wifi_umac_cmd_abort_scan {
            umac_hdr: nrf_wifi_umac_hdr::default(),
        };
        cmd.prepare();
        cmd
    }
}

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_get_scan_results,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_SCAN_RESULTS
);

impl Default for nrf_wifi_umac_cmd_get_scan_results {
    fn default() -> Self {
        let mut cmd = nrf_wifi_umac_cmd_get_scan_results {
            umac_hdr: nrf_wifi_umac_hdr::default(),
            scan_reason: scan_reason::SCAN_CONNECT as i32,
        };
        cmd.prepare();
        cmd
    }
}

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_set_power_save,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_POWER_SAVE
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_mcast_filter,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_MCAST_FILTER
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_mgmt_frame_reg,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REGISTER_FRAME
);

impl_cmd!(
    umac,
    nrf_wifi_cmd_get_wiphy,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_WIPHY
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_key,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_KEY
);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_chg_sta,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_STATION
);

// TODO: this is a wild guess.
const MAX_CMD_SIZE: usize = 1024;

impl<BUS: Bus> Rpu<BUS> {
    pub(crate) async fn send_command_raw(
        &mut self,
        domain: nrf_wifi_host_rpu_msg_type,
        buffer: *const [u8],
    ) -> Result<(), Error> {
        let mut buf = [0u32; MAX_CMD_SIZE / 4];
        let buf8 = slice8_mut(&mut buf);

        let header = host_rpu_msg {
            hdr: host_rpu_msg_hdr {
                len: buffer.len() as u32,
                resubmit: 0,
            },
            type_: domain as i32,
            msg: unsafe { zeroed() },
        };

        let command_type = match domain {
            nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM => {
                let header: *const nrf_wifi_sys_head = buffer as *const nrf_wifi_sys_head;
                unsafe { header.read().cmd_event }
            }
            nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC => {
                let header: *const nrf_wifi_umac_hdr = buffer as *const nrf_wifi_umac_hdr;
                unsafe { header.read().cmd_evnt }
            }
            _ => 0xDEAD_BEEF,
        };

        debug!(
            "Writing command {} to domain {:?}. Length: {}",
            command_type,
            domain,
            buffer.len()
        );

        let cmd_bytes = sliceit(&header);
        buf8[..cmd_bytes.len()].copy_from_slice(cmd_bytes);

        // TODO: Turn into function
        let src: &[u8] = unsafe { &*buffer };
        buf8[cmd_bytes.len()..(cmd_bytes.len() + buffer.len())].copy_from_slice(src);

        let total_length = buffer.len() + cmd_bytes.len();

        match embassy_time::with_timeout(
            Duration::from_secs(1),
            self.enqueue_command_and_trigger(
                slice8(&buf[..(total_length + 3) / 4]),
                NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_CTRL,
                0,
            ),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    pub(crate) async fn send_command<T: Command>(&mut self, mut command: T) -> Result<(), Error> {
        #[repr(C, packed)]
        struct Msg<T> {
            header: host_rpu_msg,
            cmd: T,
        }

        let mut buf = [0u32; MAX_CMD_SIZE / 4];
        let buf8 = slice8_mut(&mut buf);

        command.prepare();

        let kind = command.kind();

        let mut cmd = Msg {
            header: unsafe { zeroed() },
            cmd: command,
        };

        let length = size_of_val(&cmd) as _;

        cmd.header.hdr.len = length;
        cmd.header.type_ = T::MESSAGE_TYPE as _;

        let cmd_bytes = sliceit(&cmd);
        buf8[..cmd_bytes.len()].copy_from_slice(cmd_bytes);

        debug!(
            "Writing command {:#x} to domain {:?}. Length: {}",
            kind,
            T::MESSAGE_TYPE,
            length
        );

        match embassy_time::with_timeout(
            Duration::from_secs(1),
            self.enqueue_command_and_trigger(
                slice8(&buf[..(size_of::<T>() + 3) / 4]),
                NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_CTRL,
                0,
            ),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    async fn wait_until_ready_for_new_command(&mut self, message_type: NRF_WIFI_HAL_MSG_TYPE) -> Result<(), Error> {
        if message_type != NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_CTRL {
            return Err(Error::InvalidArgument);
        }

        if let Some(hostport_queues_info) = self.hostport_queues_info {
            for _ in 0..10 {
                /* Check if any command pointers are available to post a message */
                let is_empty = self
                    .read_u32(hostport_queues_info.cmd_avl_queue.dequeue_addr, None)
                    .await
                    == 0;

                if !is_empty {
                    return Ok(());
                }

                Timer::after(Duration::from_millis(1)).await;
            }
        }

        Err(Error::Timeout)
    }

    async fn enqueue_command_and_trigger(
        &mut self,
        message: &[u8],
        message_type: NRF_WIFI_HAL_MSG_TYPE,
        queue_identifier: usize,
    ) -> Result<(), Error> {
        let hostport_queues_info = match self.hostport_queues_info {
            Some(hostport_queues_info) => Ok(hostport_queues_info),
            None => Err(Error::NotInitialized),
        }?;

        let mut bytes_left_to_send = message.len();
        let mut offset: usize = 0;

        while bytes_left_to_send > 0 {
            let bytes_to_send = if bytes_left_to_send > MAX_NRF_WIFI_UMAC_CMD_SIZE as usize {
                MAX_NRF_WIFI_UMAC_CMD_SIZE as usize
            } else {
                bytes_left_to_send
            };

            self.wait_until_ready_for_new_command(NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_CTRL)
                .await?;

            // Wait until we get an address to write to
            // This queue might already be full with other messages, so we'll just have to wait a bit
            let message_address = loop {
                if let Some(message_address) = self.hostport_queue_dequeue(hostport_queues_info.cmd_avl_queue).await {
                    break message_address;
                }
            };

            debug!(
                "Enqueued command to hostport address: {:#x} ({}/{} bytes)",
                message_address,
                offset + bytes_to_send,
                message.len()
            );

            // Write the message to the suggested address
            self.write_buffer(message_address, None, slice32(&message[offset..offset + bytes_to_send]))
                .await;

            match message_type {
                NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_CTRL
                | NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_DATA_TX => {
                    // Post the updated information to the RPU
                    self.hostport_queue_enqueue(hostport_queues_info.cmd_busy_queue, message_address)
                        .await;

                    // --- Trigger ---
                    //
                    // Indicate to the RPU that the information has been posted
                    self.write_u32(
                        RPU_REG_INT_TO_MCU_CTRL,
                        Some(ProcessorType::Umac),
                        self.num_commands | 0x7fff_0000,
                    )
                    .await;

                    self.num_commands = self.num_commands.wrapping_add(1);
                }
                NRF_WIFI_HAL_MSG_TYPE::NRF_WIFI_HAL_MSG_TYPE_CMD_DATA_RX => {
                    self.hostport_queue_enqueue(
                        hostport_queues_info.rx_buf_busy_queue[queue_identifier],
                        message_address,
                    )
                    .await;
                }
                _ => {
                    warn!("Invalid message type: {}", message_type as u32);
                    return Err(Error::InvalidArgument);
                }
            }

            bytes_left_to_send -= bytes_to_send;
            offset += bytes_to_send;
        }

        Ok(())
    }

    pub(super) async fn send_rx_command(
        &mut self,
        command: &[u32],
        descriptor_identifier: u32,
        queue_identifier: usize,
    ) -> Result<(), Error> {
        let address_base = match self.rx_command_base_address {
            Some(address_base) => Ok(address_base),
            None => Err(Error::InvalidAddress),
        }?;

        let hostport_queue_info = match self.hostport_queues_info {
            Some(hostport_queue_info) => Ok(hostport_queue_info),
            None => Err(Error::NotInitialized),
        }?;

        // TODO: need max size here

        let address = address_base + RPU_DATA_CMD_SIZE_MAX_RX * descriptor_identifier;
        let host_address = (address & RPU_ADDR_MASK_OFFSET) | RPU_MCU_CORE_INDIRECT_BASE;

        // Write the command to the core
        self.write_core(host_address, command, ProcessorType::Lmac).await;

        // Post the updated information to the RPU
        self.hostport_queue_enqueue(hostport_queue_info.rx_buf_busy_queue[queue_identifier], address)
            .await;

        Ok(())
    }
}
