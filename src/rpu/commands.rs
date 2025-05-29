use core::mem::{size_of, zeroed};

use embassy_time::Duration;

use crate::{
    bindings::{
        host_rpu_msg, host_rpu_msg_hdr, nrf_wifi_cmd_get_stats, nrf_wifi_cmd_sys_deinit, nrf_wifi_cmd_sys_init,
        nrf_wifi_host_rpu_msg_type, nrf_wifi_ie, nrf_wifi_index_ids, nrf_wifi_scan_params, nrf_wifi_sys_commands,
        nrf_wifi_sys_head, nrf_wifi_umac_cmd_abort_scan, nrf_wifi_umac_cmd_add_vif, nrf_wifi_umac_cmd_change_macaddr,
        nrf_wifi_umac_cmd_chg_vif_state, nrf_wifi_umac_cmd_get_scan_results, nrf_wifi_umac_cmd_scan,
        nrf_wifi_umac_commands, nrf_wifi_umac_hdr, nrf_wifi_umac_scan_info, rpu_stats_type, scan_reason,
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
    fn fill(&mut self);

    fn kind(&self) -> nrf_wifi_host_rpu_msg_type {
        return Self::MESSAGE_TYPE;
    }
}

macro_rules! impl_cmd {
    (sys, $cmd:path, $num:expr) => {
        impl Command for $cmd {
            const MESSAGE_TYPE: nrf_wifi_host_rpu_msg_type =
                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM;
            fn fill(&mut self) {
                self.sys_head = nrf_wifi_sys_head {
                    cmd_event: $num as _,
                    len: size_of::<Self>() as _,
                };
            }
        }
    };
    (umac, $cmd:path, $num:expr) => {
        impl Command for $cmd {
            const MESSAGE_TYPE: nrf_wifi_host_rpu_msg_type =
                nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC;
            fn fill(&mut self) {
                self.umac_hdr = nrf_wifi_umac_hdr {
                    cmd_evnt: $num as _,
                    ..unsafe { zeroed() }
                };
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
        cmd.fill();
        cmd
    }
}

impl_cmd!(sys, nrf_wifi_cmd_sys_deinit, nrf_wifi_sys_commands::NRF_WIFI_CMD_DEINIT);

impl_cmd!(
    umac,
    nrf_wifi_umac_cmd_add_vif,
    nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_INTERFACE
);

// impl Default for nrf_wifi_umac_cmd_add_vif {
//     fn default() -> Self {
//         nrf_wifi_umac_cmd_add_vif {
//             umac_hdr: nrf_wifi_umac_hdr {
//                 cmd_evnt: nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_INTERFACE as u32,
//                 ids: nrf_wifi_index_ids {
//                     valid_fields: NRF_WIFI_INDEX_IDS_WDEV_ID_VALID,
//                     ifaceindex: 0,
//                     nrf_wifi_wiphy_idx: 0,
//                     // TODO: 0 is the default. Only need to send this for wdev_id != 0
//                     wdev_id: 1,
//                 },
//                 portid: 0,
//                 seq: 0,
//                 rpu_ret_val: 0,
//             },
//             valid_fields: 0,
//             info: nrf_wifi_umac_add_vif_info {
//     iftype: 0,
//     nrf_wifi_use_4addr: ,
//     mon_flags: ,
//     mac_addr: ,
//     ifacename:,
//             },
//         }
//     }
// }

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
                scan_reason: scan_reason::SCAN_CONNECT as i32,
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
                    skip_local_admin_macs: 1,
                    center_frequency: unsafe { zeroed() },
                },
            },
        };
        cmd.fill();
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
        cmd.fill();
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
        cmd.fill();
        cmd
    }
}

// TODO: this is a wild guess.
const MAX_CMD_SIZE: usize = 1024;

impl<BUS: Bus> Rpu<BUS> {
    pub(crate) async fn send_command_raw(
        &mut self,
        kind: nrf_wifi_host_rpu_msg_type,
        buffer: *const [u8],
    ) -> Result<(), Error> {
        let mut buf = [0u32; MAX_CMD_SIZE / 4];
        let buf8 = slice8_mut(&mut buf);

        let header = host_rpu_msg {
            hdr: host_rpu_msg_hdr {
                len: buffer.len() as u32,
                resubmit: 0,
            },
            type_: kind as i32,
            msg: unsafe { zeroed() },
        };

        let cmd_bytes = sliceit(&header);
        buf8[..cmd_bytes.len()].copy_from_slice(cmd_bytes);

        // TODO: Turn into function
        let src: &[u8] = unsafe { &*buffer };
        buf8[cmd_bytes.len()..(cmd_bytes.len() + buffer.len())].copy_from_slice(src);

        match embassy_time::with_timeout(
            Duration::from_secs(1),
            self.enqueue_command_and_trigger(slice8(&buf[..(buffer.len() + 3) / 4])),
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

        command.fill();

        let mut cmd = Msg {
            header: unsafe { zeroed() },
            cmd: command,
        };

        cmd.header.hdr.len = size_of_val(&cmd) as _;
        cmd.header.type_ = T::MESSAGE_TYPE as _;

        let cmd_bytes = sliceit(&cmd);
        buf8[..cmd_bytes.len()].copy_from_slice(cmd_bytes);

        match embassy_time::with_timeout(
            Duration::from_secs(1),
            self.enqueue_command_and_trigger(slice8(&buf[..(size_of::<T>() + 3) / 4])),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    async fn enqueue_command_and_trigger(&mut self, message: &[u8]) -> Result<(), Error> {
        let hostport_queues_info = match self.hostport_queues_info {
            Some(hostport_queues_info) => Ok(hostport_queues_info),
            None => Err(Error::NotInitialized),
        }?;

        // Wait until we get an address to write to
        // This queue might already be full with other messages, so we'll just have to wait a bit
        let message_address = loop {
            if let Some(message_address) = self.hostport_queue_dequeue(hostport_queues_info.cmd_avl_queue).await {
                break message_address;
            }
        };

        debug!("Writing to {:#x}", message_address);

        // Write the message to the suggested address
        self.write_buffer(message_address, None, slice32(message)).await;

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

        let address = address_base + RPU_DATA_CMD_SIZE_MAX_RX * descriptor_identifier;
        let host_address = address & RPU_ADDR_MASK_OFFSET | RPU_MCU_CORE_INDIRECT_BASE;

        // Write the command to the core
        self.write_core(host_address, command, ProcessorType::Lmac).await;

        // Post the updated information to the RPU
        self.hostport_queue_enqueue(hostport_queue_info.rx_buf_busy_queue[queue_identifier], address)
            .await;

        Ok(())
    }

    // pub(crate) async fn nrf_wifi_fmac_rx_cmd_send<'a, BUS>(
    //     bus: &mut BUS,
    //     hal_info: &nrf_wifi_hal_info,
    //     rx_buffer: &mut RxBuffer<'a>,
    //     command_type: nrf_wifi_fmac_rx_cmd_type,
    // ) -> Result<(), NrfWifiError>
    // where
    //     BUS: Bus,
    // {
    //     // let buffer_length = umac_context.rx_buf_pools[pool_info.pool_id as usize].buf_sz as u32 + RX_BUF_HEADROOM;
    //
    //     match command_type {
    //         nrf_wifi_fmac_rx_cmd_type::NRF_WIFI_FMAC_RX_CMD_TYPE_INIT => {
    //             if rx_buffer.mapped {
    //                 return Err(NrfWifiError::RxBufferAlreadyMapped);
    //             }
    //
    //             let address = hal_info.rx_cmd_base + RPU_DATA_CMD_SIZE_MAX_RX * rx_buffer.descriptor_identifier;
    //             let host_address = (address & RPU_ADDR_MASK_OFFSET) | RPU_MCU_CORE_INDIRECT_BASE;
    //
    //             let command = host_rpu_rx_buf_info {
    //                 addr: rx_buffer.data.as_ptr().cast::<()>() as u32,
    //             };
    //
    //             // Write
    //             bus_write_u32(bus, host_address, None, command.addr).await;
    //
    //             // Post
    //             bus_write_u32(bus, hal_info.hpqm_info.cmd_busy_queue.enqueue_addr, None, address).await;
    //         }
    //         nrf_wifi_fmac_rx_cmd_type::NRF_WIFI_FMAC_RX_CMD_TYPE_DEINIT => {}
    //         nrf_wifi_fmac_rx_cmd_type::NRF_WIFI_FMAC_RX_CMD_TYPE_MAX => {}
    //     }
    //
    //     Ok(())
    //
    //     // let addr_base = hal_info.rx_cmd_base;
    //     // let max_cmd_size = RPU_DATA_CMD_SIZE_MAX_RX;
    //     //
    //     // let addr = addr_base + max_cmd_size * desc_id;
    //     // let host_addr = addr & RPU_ADDR_MASK_OFFSET | RPU_MCU_CORE_INDIRECT_BASE;
    //     //
    //     // // Write the command to the core
    //     // self.rpu_write_core(host_addr, command, ProcessorType::LMAC).await; // LMAC is a guess here
    //     //
    //     // // Post the updated information to the RPU
    //     // self.rpu_hpq_enqueue(
    //     //     self.rpu_info.as_ref().unwrap().hpqm_info.rx_buf_busy_queue[pool_id],
    //     //     addr,
    //     // )
    //     // .await;
    // }
}
