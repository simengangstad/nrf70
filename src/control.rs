use core::ptr;

use embassy_time::{Duration, Timer};

use crate::{
    action::Action,
    bindings::{
        nrf_wifi_cmd_get_stats, nrf_wifi_sys_umac_event_stats, nrf_wifi_umac_cmd_abort_scan,
        nrf_wifi_umac_cmd_get_scan_results, nrf_wifi_umac_cmd_scan,
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

impl<'a> Control<'a> {
    pub async fn init(&mut self, firmware: &'static [u8]) -> Result<(), Error> {
        self.action_state
            .issue(Action::LoadFirmware(firmware))
            .await
            .map(|_| ())
    }

    pub async fn scan(&mut self, options: ScanOptions) -> Result<(), Error> {
        let mut command = nrf_wifi_umac_cmd_scan::default();
        command.fill();

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

        // TEMP

        // command.info.scan_params.num_scan_ssids = 2;
        let mut response = [0u8; 1024];

        match self
            .action_state
            .issue(Action::Command((
                command.kind(),
                false,
                sliceit(&command),
                Some(&mut response[..]),
            )))
            .await
        {
            Ok(_) => (),
            Err(error) => {
                error!("Failed to get stats: {:?}", error);
                return Err(error);
            }
        }

        Timer::after(Duration::from_millis(3000)).await;

        let command = nrf_wifi_umac_cmd_get_scan_results::default();

        match self
            .action_state
            .issue(Action::Command((
                command.kind(),
                false,
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

        // let command = nrf_wifi_umac_cmd_abort_scan::default();
        //
        // match self
        //     .action_state
        //     .issue(Action::Command((
        //         command.kind(),
        //         false,
        //         sliceit(&command),
        //         Some(&mut response[..]),
        //     )))
        //     .await
        // {
        //     Ok(response_length) => {
        //         if let Some(length) = response_length {
        //             info!("Got length {}", length);
        //         }
        //     }
        //     Err(error) => {
        //         error!("Failed to get stats: {:?}", error);
        //         return Err(error);
        //     }
        // }

        self.get_stats().await
    }

    // pub async fn get_scan_results(&mut self) -> Result<(), Error> {
    //     self.action_state.issue(Action::GetScanResults).await.map(|_| ())
    // }

    pub async fn get_stats(&mut self) -> Result<(), Error> {
        let mut command = nrf_wifi_cmd_get_stats::default();
        command.fill();

        let mut response = [0u8; 1024];

        match self
            .action_state
            .issue(Action::Command((
                command.kind(),
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
