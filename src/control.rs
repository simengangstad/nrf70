use core::ptr;

use embassy_time::Duration;

use crate::{
    action::{Action, Item},
    bindings::{
        host_rpu_umac_info, nrf_wifi_cmd_get_stats, nrf_wifi_sys_umac_event_stats, nrf_wifi_umac_change_macaddr_info,
        nrf_wifi_umac_cmd_change_macaddr, nrf_wifi_umac_cmd_chg_vif_state, nrf_wifi_umac_cmd_get_scan_results,
        nrf_wifi_umac_cmd_scan, nrf_wifi_umac_hdr,
    },
    rpu::commands::Command,
    util::sliceit,
    Control, Error,
};

/// WiFi scan type.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanType {
    /// Active scan: the station actively transmits probes that make APs respond.
    /// Faster, but uses more power.
    Active,
    /// Passive scan: the station doesn't transmit any probes, just listens for beacons.
    /// Slower, but uses less power.
    Passive,
}

/// Scan options.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct ScanOptions {
    /// SSID to scan for.
    // pub ssid: Option<heapless::String<32>>,
    /// If set to `None`, all APs will be returned. If set to `Some`, only APs
    /// with the specified BSSID will be returned.
    pub bssid: Option<[u8; 6]>,
    /// Number of probes to send on each channel.
    pub nprobes: Option<u16>,
    /// Time to spend waiting on the home channel.
    pub home_time: Option<Duration>,
    /// Scan type: active or passive.
    pub scan_type: ScanType,
    /// Period of time to wait on each channel when passive scanning.
    pub dwell_time: Option<Duration>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            bssid: None,
            nprobes: None,
            home_time: None,
            scan_type: ScanType::Passive,
            dwell_time: None,
        }
    }
}

#[allow(dead_code)]
impl<'a> Control<'a> {
    pub async fn init(&mut self, firmware: &'static [u8]) -> Result<(), Error> {
        self.action_state.issue(Action::Boot(firmware)).await.map(|_| ())?;
        info!("Boot done");

        // --- Update MAC address ---

        let mut umac_info_buffer = [0u8; size_of::<host_rpu_umac_info>()];

        self.action_state
            .issue(Action::Get((Item::UmacInfo, &mut umac_info_buffer[..])))
            .await?;

        let umac_info: host_rpu_umac_info = unsafe { core::mem::transmute_copy(&umac_info_buffer) };

        let mac_address = [
            (umac_info.mac_address0[0]) as u8,
            (umac_info.mac_address0[0] >> 8) as u8,
            (umac_info.mac_address0[0] >> 16) as u8,
            0x0,
            (umac_info.mac_address0[1]) as u8,
            (umac_info.mac_address0[1] >> 8) as u8,
        ];

        let header = nrf_wifi_umac_hdr::default();
        let mut command = nrf_wifi_umac_cmd_change_macaddr {
            umac_hdr: header,
            macaddr_info: nrf_wifi_umac_change_macaddr_info { mac_addr: mac_address },
        };
        command.prepare();

        match self
            .action_state
            .issue(Action::Command((command.domain(), true, sliceit(&command), None)))
            .await
        {
            Ok(_) => {}
            Err(error) => {
                error!("Failed to set MAC address: {:?}", error);
                return Err(error);
            }
        }

        info!(
            "Set MAC address for interface: {:#x}:{:#x}:{:#x}:{:#x}:{:#x}:{:#x}",
            mac_address[0], mac_address[1], mac_address[2], mac_address[3], mac_address[4], mac_address[5]
        );

        // --- Bring interface up ---

        let mut command = nrf_wifi_umac_cmd_chg_vif_state::default();
        command.info.state = 1;
        command.prepare();

        match self
            .action_state
            .issue(Action::Command((command.domain(), true, sliceit(&command), None)))
            .await
        {
            Ok(_) => {}
            Err(error) => {
                error!("Failed to change interface state: {:?}", error);
                return Err(error);
            }
        }

        info!("Brought interface up");

        // let result = self.read_u32_from_region(SYSBUS, 0x0C0).await;
        // info!("PART: {}", result);

        // --- Set mcast address ---
        // TODO

        Ok(())
    }

    pub async fn scan(&mut self, options: ScanOptions) -> Result<(), Error> {
        let mut command = nrf_wifi_umac_cmd_scan::default();

        match options.scan_type {
            ScanType::Active => {
                command.info.scan_params.passive_scan = 0;

                if let Some(dwell_time) = options.dwell_time {
                    command.info.scan_params.dwell_time_active = dwell_time.as_millis() as u16;
                }
            }
            ScanType::Passive => {
                command.info.scan_params.passive_scan = 1;

                if let Some(dwell_time) = options.dwell_time {
                    command.info.scan_params.dwell_time_passive = dwell_time.as_millis() as u16;
                }
            }
        }

        if let Some(bssid) = options.bssid {
            command.info.scan_params.mac_addr = bssid;
        }

        match self
            .action_state
            .issue(Action::Command((command.domain(), false, sliceit(&command), None)))
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                error!("Failed to get stats: {:?}", error);
                return Err(error);
            }
        }
    }

    pub async fn get_scan_results(&mut self) -> Result<(), Error> {
        let command = nrf_wifi_umac_cmd_get_scan_results::default();

        let mut response = [0u8; 1024];

        match self
            .action_state
            .issue(Action::Command((
                command.domain(),
                true,
                sliceit(&command),
                Some(&mut response[..]),
            )))
            .await
        {
            Ok(response_length) => {
                if let Some(length) = response_length {
                    info!("Response length: {}", length);
                }
            }
            Err(error) => {
                error!("Failed to get stats: {:?}", error);
                return Err(error);
            }
        }

        Ok(())
    }

    pub async fn get_stats(&mut self) -> Result<(), Error> {
        let command = nrf_wifi_cmd_get_stats::default();

        let mut response = [0u8; 1024];

        match self
            .action_state
            .issue(Action::Command((
                command.domain(),
                true,
                sliceit(&command),
                Some(&mut response[..]),
            )))
            .await
        {
            Ok(response_length) => {
                if let Some(length) = response_length {
                    let data: nrf_wifi_sys_umac_event_stats = unsafe { ptr::read(response.as_ptr() as *const _) };
                    info!("{}: {:?}", length, data);
                }

                Ok(())
            }
            Err(error) => {
                error!("Failed to get stats: {:?}", error);
                Err(error)
            }
        }
    }
}
