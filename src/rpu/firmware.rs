use core::{
    fmt,
    mem::{self, zeroed},
};

use embassy_time::Timer;

use crate::{
    bindings::*,
    bus::Bus,
    remap_global_addr_to_region_and_offset,
    rpu::{MAX_TX_AGGREGATION, RX_MAX_DATA_SIZE},
    util::slice32,
    Error,
};

use super::{ProcessorType, Rpu, RX_BUFS_PER_QUEUE};

#[derive(Copy, Clone)]
pub struct FirmwareImage<'a> {
    pub data: &'a [u8],
    pub kind: nrf70_image_ids,
}

impl<'a> FirmwareImage<'a> {
    pub fn destination_address(&self) -> u32 {
        match self.kind {
            nrf70_image_ids::NRF70_IMAGE_UMAC_PRI => RPU_MEM_UMAC_PATCH_BIMG,
            nrf70_image_ids::NRF70_IMAGE_UMAC_SEC => RPU_MEM_UMAC_PATCH_BIN,

            nrf70_image_ids::NRF70_IMAGE_LMAC_PRI => RPU_MEM_LMAC_PATCH_BIMG,
            nrf70_image_ids::NRF70_IMAGE_LMAC_SEC => RPU_MEM_LMAC_PATCH_BIN,
        }
    }

    pub fn processor(&self) -> ProcessorType {
        match self.kind {
            nrf70_image_ids::NRF70_IMAGE_UMAC_PRI | nrf70_image_ids::NRF70_IMAGE_UMAC_SEC => ProcessorType::Umac,
            nrf70_image_ids::NRF70_IMAGE_LMAC_PRI | nrf70_image_ids::NRF70_IMAGE_LMAC_SEC => ProcessorType::Lmac,
        }
    }
}

pub struct FirmwareInfo<'a> {
    pub images: [Option<FirmwareImage<'a>>; 4],

    #[allow(dead_code)]
    pub features: nrf70_feature_flags,
}

impl<'a> FirmwareInfo<'a> {
    pub fn read(blob: *const [u8]) -> Result<Self, Error> {
        if blob.len() < core::mem::size_of::<nrf70_fw_image_info>() {
            return Err(Error::FirmwareParseError(FirmwareParseError::BufferTooSmall));
        }

        debug!("Parsing firmware binary blob...");

        let info = blob.cast::<nrf70_fw_image_info>();

        let mut firmware_info = FirmwareInfo {
            images: [None; 4],
            features: unsafe {
                nrf70_feature_flags::try_from((*info).feature_flags)
                    .map_err(|error| Error::FirmwareParseError(FirmwareParseError::InvalidFeatureFlags(error)))?
            },
        };

        let signature = unsafe { (*info).signature };
        let number_of_images = unsafe { (*info).num_images };

        if signature != NRF_WIFI_PATCH_SIGNATURE {
            return Err(Error::FirmwareParseError(FirmwareParseError::InvalidSignature));
        }

        if number_of_images != NRF_WIFI_PATCH_NUM_IMAGES {
            return Err(Error::FirmwareParseError(FirmwareParseError::NotEnoughImages));
        }

        let data_length = unsafe { (*info).len } as usize;

        let mut data_offset: usize = 0;

        for image_index in 0..number_of_images as usize {
            let image = unsafe { (*info).data.as_ptr().add(data_offset).cast::<nrf70_fw_image>() };

            let image_type = unsafe { (*image).type_ };
            let image_length = unsafe { (*image).len } as usize;
            let image_data = unsafe { &(*image).data };

            let image_type = match nrf70_image_ids::try_from(image_type) {
                Ok(image_type) => Ok(image_type),
                Err(_) => Err(Error::FirmwareParseError(FirmwareParseError::InvalidImageType)),
            }?;

            debug!(
                "Image {}, type: {:?}, length: {}",
                image_index, image_type, image_length,
            );

            firmware_info.images[image_index] = Some(FirmwareImage {
                data: unsafe { image_data.as_slice(image_length) },
                kind: image_type,
            });

            data_offset += image_length + core::mem::size_of::<nrf70_fw_image>();
        }

        if data_length != data_offset {
            return Err(Error::FirmwareParseError(FirmwareParseError::InvalidDataLength));
        }

        Ok(firmware_info)
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirmwareParseError {
    BufferTooSmall,

    /// The calculated data length from the images in the payload does not match the data length in
    /// the payload
    InvalidDataLength,

    InvalidSignature,

    NotEnoughImages,

    InvalidImageType,

    InvalidFeatureFlags(u32),
}

impl fmt::Display for FirmwareParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FirmwareParseError::BufferTooSmall => write!(f, "buffer too small"),
            FirmwareParseError::InvalidDataLength => write!(f, "invalid data length"),
            FirmwareParseError::InvalidSignature => write!(f, "invalid signature"),
            FirmwareParseError::NotEnoughImages => write!(f, "not enough images"),
            FirmwareParseError::InvalidImageType => write!(f, "invalid image type"),
            FirmwareParseError::InvalidFeatureFlags(value) => write!(f, "invalid feature flags {value}"),
        }
    }
}

pub struct FirmwareVersion {
    pub version: u8,
    pub major: u8,
    pub minor: u8,
    pub extra: u8,
}

impl<BUS: Bus> Rpu<BUS> {
    pub(super) async fn firmware_load<'firmware_info_lifetime>(
        &mut self,
        firmware_info: &FirmwareInfo<'firmware_info_lifetime>,
    ) {
        const CHUNK_SIZE: usize = 1024;

        for image in firmware_info.images.into_iter().flatten() {
            debug!("Loading patch for {}", image.kind);

            let (memory_region, offset) =
                remap_global_addr_to_region_and_offset(image.destination_address(), Some(image.processor()));

            for (i, chunk) in image.data.chunks(CHUNK_SIZE).enumerate() {
                let chunk_offset = offset + (CHUNK_SIZE * i) as u32;

                self.write_buffer_to_region(memory_region, chunk_offset, slice32(chunk))
                    .await;
            }
        }
    }

    pub(super) async fn firmware_boot(&mut self) {
        // This will block until the boot signatures are verified.
        let processsors = [ProcessorType::Lmac, ProcessorType::Umac];

        for processor in processsors {
            let boot_signature_address = match processor {
                ProcessorType::Lmac => RPU_MEM_LMAC_BOOT_SIG,
                ProcessorType::Umac => RPU_MEM_UMAC_BOOT_SIG,
            };

            // Clear the firmware pass signature location
            self.write_u32(boot_signature_address, Some(processor), 0).await;

            self.write_u32(
                match processor {
                    ProcessorType::Lmac => RPU_REG_UCC_SLEEP_CTRL_DATA_0,
                    ProcessorType::Umac => RPU_REG_UCC_SLEEP_CTRL_DATA_1,
                },
                Some(processor),
                match processor {
                    ProcessorType::Lmac => NRF_WIFI_LMAC_ROM_PATCH_OFFSET,
                    ProcessorType::Umac => NRF_WIFI_UMAC_ROM_PATCH_OFFSET,
                },
            )
            .await;

            // Write the boot vectors
            let boot_vectors = match processor {
                ProcessorType::Lmac => [
                    [RPU_REG_MIPS_MCU_BOOT_EXCP_INSTR_0, NRF_WIFI_LMAC_BOOT_EXCP_VECT_0],
                    [RPU_REG_MIPS_MCU_BOOT_EXCP_INSTR_1, NRF_WIFI_LMAC_BOOT_EXCP_VECT_1],
                    [RPU_REG_MIPS_MCU_BOOT_EXCP_INSTR_2, NRF_WIFI_LMAC_BOOT_EXCP_VECT_2],
                    [RPU_REG_MIPS_MCU_BOOT_EXCP_INSTR_3, NRF_WIFI_LMAC_BOOT_EXCP_VECT_3],
                ],
                ProcessorType::Umac => [
                    [RPU_REG_MIPS_MCU2_BOOT_EXCP_INSTR_0, NRF_WIFI_UMAC_BOOT_EXCP_VECT_0],
                    [RPU_REG_MIPS_MCU2_BOOT_EXCP_INSTR_1, NRF_WIFI_UMAC_BOOT_EXCP_VECT_1],
                    [RPU_REG_MIPS_MCU2_BOOT_EXCP_INSTR_2, NRF_WIFI_UMAC_BOOT_EXCP_VECT_2],
                    [RPU_REG_MIPS_MCU2_BOOT_EXCP_INSTR_3, NRF_WIFI_UMAC_BOOT_EXCP_VECT_3],
                ],
            };

            for boot_vector in boot_vectors {
                self.write_u32(boot_vector[0], Some(processor), boot_vector[1]).await;
            }

            // Perform pulsed soft reset
            self.write_u32(
                match processor {
                    ProcessorType::Lmac => RPU_REG_MIPS_MCU_CONTROL,
                    ProcessorType::Umac => RPU_REG_MIPS_MCU2_CONTROL,
                },
                Some(processor),
                0x1,
            )
            .await;

            // Check boot signature
            let expected_boot_signature = match processor {
                ProcessorType::Lmac => NRF_WIFI_LMAC_BOOT_SIG,
                ProcessorType::Umac => NRF_WIFI_UMAC_BOOT_SIG,
            };

            while self.read_u32(boot_signature_address, Some(processor)).await != expected_boot_signature {
                Timer::after_millis(10).await;
            }
        }
    }

    pub(super) async fn firmware_initialize(&mut self, rf_parameters: &nrf_wifi_phy_rf_params) -> Result<(), Error> {
        let init_command = nrf_wifi_cmd_sys_init {
            sys_head: unsafe { zeroed() },
            wdev_id: 0,
            sys_params: nrf_wifi_sys_params {
                sleep_enable: 0, // TODO: for low power
                hw_bringup_time: HW_DELAY,
                sw_bringup_time: SW_DELAY,
                bcn_time_out: BCN_TIMEOUT,
                calib_sleep_clk: CALIB_SLEEP_CLOCK_ENABLE,
                phy_calib: NRF_WIFI_DEF_PHY_CALIB,
                mac_addr: [0; 6],
                rf_params: unsafe { mem::transmute(*rf_parameters) },
                rf_params_valid: 1,
            },
            rx_buf_pools: [
                rx_buf_pool_params {
                    buf_sz: RX_MAX_DATA_SIZE as _, // TODO is this including the header or not?
                    num_bufs: RX_BUFS_PER_QUEUE as _,
                },
                rx_buf_pool_params {
                    buf_sz: RX_MAX_DATA_SIZE as _, // TODO is this including the header or not?
                    num_bufs: RX_BUFS_PER_QUEUE as _,
                },
                rx_buf_pool_params {
                    buf_sz: RX_MAX_DATA_SIZE as _, // TODO is this including the header or not?
                    num_bufs: RX_BUFS_PER_QUEUE as _,
                },
            ],
            data_config_params: nrf_wifi_data_config_params {
                rate_protection_type: 0,
                aggregation: 1,
                wmm: 1,
                max_num_tx_agg_sessions: 4,
                max_num_rx_agg_sessions: 8,
                max_tx_aggregation: MAX_TX_AGGREGATION as _,
                reorder_buf_size: 64,
                max_rxampdu_size: 3,
            },
            temp_vbat_config_params: temp_vbat_config {
                temp_based_calib_en: NRF_WIFI_TEMP_CALIB_ENABLE,
                temp_calib_bitmap: NRF_WIFI_DEF_PHY_TEMP_CALIB,
                vbat_calibp_bitmap: NRF_WIFI_DEF_PHY_VBAT_CALIB,
                temp_vbat_mon_period: NRF_WIFI_TEMP_CALIB_PERIOD,
                vth_very_low: NRF_WIFI_VBAT_VERYLOW as _,
                vth_low: NRF_WIFI_VBAT_LOW as _,
                vth_hi: NRF_WIFI_VBAT_HIGH as _,
                temp_threshold: NRF_WIFI_TEMP_CALIB_THRESHOLD as _,
                vbat_threshold: 0,
            },
            country_code: [0, 0],
            op_band: op_band::BAND_ALL as u32, // TODO: should be configuration
            tcp_ip_checksum_offload: 0,
            mgmt_buff_offload: 0,
            feature_flags: 0,
            coex_disable_ptiwin_for_wifi_scan: 0,
            disable_beamforming: 0,
            discon_timeout: 20,
            display_scan_bss_limit: 150,
            ps_exit_strategy: ps_exit_strategy::EVERY_TIM as u8,
            watchdog_timer_val: 0xFF_FFFF, // TODO: enable watchdog timer
            keep_alive_enable: 1,
            keep_alive_period: 60,
            max_ps_poll_fail_cnt: 10,
            raw_scan_enable: 0,
            stbc_enable_in_ht: 0,
        };

        self.send_command(init_command).await
    }

    pub async fn firmware_version(&mut self) -> FirmwareVersion {
        let version = self.read_u32(RPU_MEM_UMAC_VER, None).await;

        FirmwareVersion {
            version: ((version & 0xFF00_0000) >> 24) as u8,
            major: ((version & 0x00FF_0000) >> 16) as u8,
            minor: ((version & 0x0000_FF00) >> 8) as u8,
            extra: (version & 0x0000_00FF) as u8,
        }
    }
}
