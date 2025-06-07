use core::cmp::min;

use crate::{
    bindings::{
        ft_prog_ver, host_rpu_umac_info, nrf_wifi_pd_adst_val, nrf_wifi_phy_rf_params, nrf_wifi_rx_gain_offset,
        nrf_wifi_temp_volt_depend_params, nrf_wifi_tx_pwr_ceil, nrf_wifi_tx_pwr_ceil_params,
        nrf_wifi_tx_pwr_systm_offset, nrf_wifi_xo_freq_offset, CALIB_XO_FLAG_MASK, CSP_HB_MAX_PWR_BKF_HI_TEMP,
        CSP_HB_MAX_PWR_BKF_LOW_TEMP, CSP_HB_VBT_LT_LOW, CSP_HB_VBT_LT_VLOW, CSP_LB_MAX_PWR_BKF_HI_TEMP,
        CSP_LB_MAX_PWR_BKF_LOW_TEMP, CSP_LB_VBT_LT_LOW, CSP_LB_VBT_LT_VLOW, CSP_MAX_CHIP_TEMP, CSP_MAX_TX_PWR_DSSS,
        CSP_MAX_TX_PWR_HB_HIGH_CHAN_MCS0, CSP_MAX_TX_PWR_HB_HIGH_CHAN_MCS7, CSP_MAX_TX_PWR_HB_LOW_CHAN_MCS0,
        CSP_MAX_TX_PWR_HB_LOW_CHAN_MCS7, CSP_MAX_TX_PWR_HB_MID_CHAN_MCS0, CSP_MAX_TX_PWR_HB_MID_CHAN_MCS7,
        CSP_MAX_TX_PWR_LB_MCS0, CSP_MAX_TX_PWR_LB_MCS7, CSP_MIN_CHIP_TEMP, CSP_PACKAGE_INFO,
        CSP_SYSTEM_OFFSET_HB_CHAN_HIGH, CSP_SYSTEM_OFFSET_HB_CHAN_LOW, CSP_SYSTEM_OFFSET_HB_CHAN_MID,
        CSP_SYSTEM_OFFSET_LB, CSP_XO_VAL, CTRL_PWR_OPTIMIZATIONS, EDGE_BACKOFF_OFFSETS,
        FT_PROG_VER1_2G_DSSS_TXCEIL_BKOFF, FT_PROG_VER1_2G_OFDM_TXCEIL_BKOFF, FT_PROG_VER1_5G_HIGH_OFDM_TXCEIL_BKOFF,
        FT_PROG_VER1_5G_LOW_OFDM_TXCEIL_BKOFF, FT_PROG_VER1_5G_MID_OFDM_TXCEIL_BKOFF,
        FT_PROG_VER2_2G_DSSS_TXCEIL_BKOFF, FT_PROG_VER2_2G_OFDM_TXCEIL_BKOFF, FT_PROG_VER2_5G_HIGH_OFDM_TXCEIL_BKOFF,
        FT_PROG_VER2_5G_LOW_OFDM_TXCEIL_BKOFF, FT_PROG_VER2_5G_MID_OFDM_TXCEIL_BKOFF,
        FT_PROG_VER3_2G_DSSS_TXCEIL_BKOFF, FT_PROG_VER3_2G_OFDM_TXCEIL_BKOFF, FT_PROG_VER3_5G_HIGH_OFDM_TXCEIL_BKOFF,
        FT_PROG_VER3_5G_LOW_OFDM_TXCEIL_BKOFF, FT_PROG_VER3_5G_MID_OFDM_TXCEIL_BKOFF, FT_PROG_VER_MASK,
        NRF_WIFI_RF_PARAMS_CONF_SIZE, NRF_WIFI_SYS_DEF_RF_PARAMS, OTP_OFF_CALIB_XO, PD_ADJUST_VAL,
        QFN_HB_MAX_PWR_BKF_HI_TEMP, QFN_HB_MAX_PWR_BKF_LOW_TEMP, QFN_HB_VBT_LT_LOW, QFN_HB_VBT_LT_VLOW,
        QFN_LB_MAX_PWR_BKF_HI_TEMP, QFN_LB_MAX_PWR_BKF_LOW_TEMP, QFN_LB_VBT_LT_LOW, QFN_LB_VBT_LT_VLOW,
        QFN_MAX_CHIP_TEMP, QFN_MAX_TX_PWR_DSSS, QFN_MAX_TX_PWR_HB_HIGH_CHAN_MCS0, QFN_MAX_TX_PWR_HB_HIGH_CHAN_MCS7,
        QFN_MAX_TX_PWR_HB_LOW_CHAN_MCS0, QFN_MAX_TX_PWR_HB_LOW_CHAN_MCS7, QFN_MAX_TX_PWR_HB_MID_CHAN_MCS0,
        QFN_MAX_TX_PWR_HB_MID_CHAN_MCS7, QFN_MAX_TX_PWR_LB_MCS0, QFN_MAX_TX_PWR_LB_MCS7, QFN_MIN_CHIP_TEMP,
        QFN_SYSTEM_OFFSET_HB_CHAN_HIGH, QFN_SYSTEM_OFFSET_HB_CHAN_LOW, QFN_SYSTEM_OFFSET_HB_CHAN_MID,
        QFN_SYSTEM_OFFSET_LB, QFN_XO_VAL, RPU_MEM_OTP_FT_PROG_VERSION, RPU_MEM_OTP_PACKAGE_TYPE,
        RX_GAIN_OFFSET_HB_HIGH_CHAN, RX_GAIN_OFFSET_HB_LOW_CHAN, RX_GAIN_OFFSET_HB_MID_CHAN,
    },
    bus::Bus,
};

use super::Rpu;

fn from_ascii_hex_digit(ascii_hex: u8) -> u8 {
    match ascii_hex {
        b'0'..=b'9' => ascii_hex - b'0',
        b'a'..=b'f' => 10 + (ascii_hex - b'a'),
        b'A'..=b'F' => 10 + (ascii_hex - b'A'),
        _ => 0,
    }
}

#[allow(clippy::cast_possible_truncation)]
impl nrf_wifi_phy_rf_params {
    fn default_from(package_type: u32) -> Self {
        let mut phy_rf_params = nrf_wifi_phy_rf_params {
            reserved: [0; 6],
            xo_offset: nrf_wifi_xo_freq_offset {
                xo_freq_offset: match package_type {
                    CSP_PACKAGE_INFO => CSP_XO_VAL as u8,
                    _ => QFN_XO_VAL as u8,
                },
            },
            pd_adjust_val: nrf_wifi_pd_adst_val {
                pd_adjt_lb_chan: PD_ADJUST_VAL as i8,
                pd_adjt_hb_low_chan: PD_ADJUST_VAL as i8,
                pd_adjt_hb_mid_chan: PD_ADJUST_VAL as i8,
                pd_adjt_hb_high_chan: PD_ADJUST_VAL as i8,
            },
            syst_tx_pwr_offset: match package_type {
                CSP_PACKAGE_INFO => nrf_wifi_tx_pwr_systm_offset {
                    syst_off_lb_chan: CSP_SYSTEM_OFFSET_LB as i8,
                    syst_off_hb_low_chan: CSP_SYSTEM_OFFSET_HB_CHAN_LOW as i8,
                    syst_off_hb_mid_chan: CSP_SYSTEM_OFFSET_HB_CHAN_MID as i8,
                    syst_off_hb_high_chan: CSP_SYSTEM_OFFSET_HB_CHAN_HIGH as i8,
                },
                _ => nrf_wifi_tx_pwr_systm_offset {
                    syst_off_lb_chan: QFN_SYSTEM_OFFSET_LB as i8,
                    syst_off_hb_low_chan: QFN_SYSTEM_OFFSET_HB_CHAN_LOW as i8,
                    syst_off_hb_mid_chan: QFN_SYSTEM_OFFSET_HB_CHAN_MID as i8,
                    syst_off_hb_high_chan: QFN_SYSTEM_OFFSET_HB_CHAN_HIGH as i8,
                },
            },
            max_pwr_ceil: match package_type {
                CSP_PACKAGE_INFO => nrf_wifi_tx_pwr_ceil {
                    max_dsss_pwr: CSP_MAX_TX_PWR_DSSS as i8,
                    max_lb_mcs7_pwr: CSP_MAX_TX_PWR_LB_MCS7 as i8,
                    max_lb_mcs0_pwr: CSP_MAX_TX_PWR_LB_MCS0 as i8,
                    max_hb_low_chan_mcs7_pwr: CSP_MAX_TX_PWR_HB_LOW_CHAN_MCS7 as i8,
                    max_hb_mid_chan_mcs7_pwr: CSP_MAX_TX_PWR_HB_MID_CHAN_MCS7 as i8,
                    max_hb_high_chan_mcs7_pwr: CSP_MAX_TX_PWR_HB_HIGH_CHAN_MCS7 as i8,
                    max_hb_low_chan_mcs0_pwr: CSP_MAX_TX_PWR_HB_LOW_CHAN_MCS0 as i8,
                    max_hb_mid_chan_mcs0_pwr: CSP_MAX_TX_PWR_HB_MID_CHAN_MCS0 as i8,
                    max_hb_high_chan_mcs0_pwr: CSP_MAX_TX_PWR_HB_HIGH_CHAN_MCS0 as i8,
                },
                _ => nrf_wifi_tx_pwr_ceil {
                    max_dsss_pwr: QFN_MAX_TX_PWR_DSSS as i8,
                    max_lb_mcs7_pwr: QFN_MAX_TX_PWR_LB_MCS7 as i8,
                    max_lb_mcs0_pwr: QFN_MAX_TX_PWR_LB_MCS0 as i8,
                    max_hb_low_chan_mcs7_pwr: QFN_MAX_TX_PWR_HB_LOW_CHAN_MCS7 as i8,
                    max_hb_mid_chan_mcs7_pwr: QFN_MAX_TX_PWR_HB_MID_CHAN_MCS7 as i8,
                    max_hb_high_chan_mcs7_pwr: QFN_MAX_TX_PWR_HB_HIGH_CHAN_MCS7 as i8,
                    max_hb_low_chan_mcs0_pwr: QFN_MAX_TX_PWR_HB_LOW_CHAN_MCS0 as i8,
                    max_hb_mid_chan_mcs0_pwr: QFN_MAX_TX_PWR_HB_MID_CHAN_MCS0 as i8,
                    max_hb_high_chan_mcs0_pwr: QFN_MAX_TX_PWR_HB_HIGH_CHAN_MCS0 as i8,
                },
            },
            rx_gain_offset: nrf_wifi_rx_gain_offset {
                rx_gain_lb_chan: CTRL_PWR_OPTIMIZATIONS as i8,
                rx_gain_hb_low_chan: RX_GAIN_OFFSET_HB_LOW_CHAN as i8,
                rx_gain_hb_mid_chan: RX_GAIN_OFFSET_HB_MID_CHAN as i8,
                rx_gain_hb_high_chan: RX_GAIN_OFFSET_HB_HIGH_CHAN as i8,
            },
            temp_volt_backoff: match package_type {
                CSP_PACKAGE_INFO => nrf_wifi_temp_volt_depend_params {
                    max_chip_temp: CSP_MAX_CHIP_TEMP as i8,
                    min_chip_temp: CSP_MIN_CHIP_TEMP as i8,
                    lb_max_pwr_bkf_hi_temp: CSP_LB_MAX_PWR_BKF_HI_TEMP as i8,
                    lb_max_pwr_bkf_low_temp: CSP_LB_MAX_PWR_BKF_LOW_TEMP as i8,
                    hb_max_pwr_bkf_hi_temp: CSP_HB_MAX_PWR_BKF_HI_TEMP as i8,
                    hb_max_pwr_bkf_low_temp: CSP_HB_MAX_PWR_BKF_LOW_TEMP as i8,
                    lb_vbt_lt_vlow: CSP_LB_VBT_LT_VLOW as i8,
                    hb_vbt_lt_vlow: CSP_HB_VBT_LT_VLOW as i8,
                    lb_vbt_lt_low: CSP_LB_VBT_LT_LOW as i8,
                    hb_vbt_lt_low: CSP_HB_VBT_LT_LOW as i8,
                    reserved: [0, 0, 0, 0],
                },
                _ => nrf_wifi_temp_volt_depend_params {
                    max_chip_temp: QFN_MAX_CHIP_TEMP as i8,
                    min_chip_temp: QFN_MIN_CHIP_TEMP as i8,
                    lb_max_pwr_bkf_hi_temp: QFN_LB_MAX_PWR_BKF_HI_TEMP as i8,
                    lb_max_pwr_bkf_low_temp: QFN_LB_MAX_PWR_BKF_LOW_TEMP as i8,
                    hb_max_pwr_bkf_hi_temp: QFN_HB_MAX_PWR_BKF_HI_TEMP as i8,
                    hb_max_pwr_bkf_low_temp: QFN_HB_MAX_PWR_BKF_LOW_TEMP as i8,
                    lb_vbt_lt_vlow: QFN_LB_VBT_LT_VLOW as i8,
                    hb_vbt_lt_vlow: QFN_HB_VBT_LT_VLOW as i8,
                    lb_vbt_lt_low: QFN_LB_VBT_LT_LOW as i8,
                    hb_vbt_lt_low: QFN_HB_VBT_LT_LOW as i8,
                    reserved: [0; 4],
                },
            },
            phy_params: [0; 158],
        };

        // Populate the default RF parameters
        for i in 0..NRF_WIFI_SYS_DEF_RF_PARAMS.len() / 2 {
            let upper_nibble = from_ascii_hex_digit(NRF_WIFI_SYS_DEF_RF_PARAMS[2 * i]);
            let lower_nibble = from_ascii_hex_digit(NRF_WIFI_SYS_DEF_RF_PARAMS[2 * i + 1]);

            phy_rf_params.phy_params[i] = (upper_nibble << 4) | lower_nibble;
        }

        phy_rf_params
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
impl<BUS: Bus> Rpu<BUS> {
    /// Get the RF parameters to be programmed to the RPU.
    ///
    /// This function is used to fetch RF parameters information from the RPU and
    /// update the default RF parameter with the OTP values.
    pub(super) async fn get_rf_parameters(
        &mut self,
        umac_info: &host_rpu_umac_info,
        otp_flags: u32,
        tx_pwr_ceil_params: &nrf_wifi_tx_pwr_ceil_params,
    ) -> nrf_wifi_phy_rf_params {
        const RF_PARAM_OFFSET: usize =
            EDGE_BACKOFF_OFFSETS::BAND_2G_LW_ED_BKF_DSSS_OFST as usize - NRF_WIFI_RF_PARAMS_CONF_SIZE as usize;

        let ft_prog_version = (self.read_u32(RPU_MEM_OTP_FT_PROG_VERSION, None).await & FT_PROG_VER_MASK) >> 16;

        let package_type = self.read_u32(RPU_MEM_OTP_PACKAGE_TYPE, None).await;
        let mut phy_rf_params = nrf_wifi_phy_rf_params::default_from(package_type);

        // Then populate the configuration based ones
        RfParameters::default()
            .populate_slice(&mut phy_rf_params.phy_params[RF_PARAM_OFFSET..(RF_PARAM_OFFSET + RF_PARAM_LENGTH)]);

        if (otp_flags & (!(CALIB_XO_FLAG_MASK as u32))) != 0 {
            phy_rf_params.xo_offset.xo_freq_offset = umac_info.calib[OTP_OFF_CALIB_XO as usize] as u8;
        }

        let (backoff_2g_dsss, backoff_2g_ofdm, backoff_5g_lowband, backoff_5g_midband, backoff_5g_highband) = {
            if let Some(ft_prog_version) = ft_prog_ver::from_u32(ft_prog_version) {
                match ft_prog_version {
                    ft_prog_ver::FT_PROG_VER1 => (
                        FT_PROG_VER1_2G_DSSS_TXCEIL_BKOFF as i8,
                        FT_PROG_VER1_2G_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER1_5G_LOW_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER1_5G_MID_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER1_5G_HIGH_OFDM_TXCEIL_BKOFF as i8,
                    ),
                    ft_prog_ver::FT_PROG_VER2 => (
                        FT_PROG_VER2_2G_DSSS_TXCEIL_BKOFF as i8,
                        FT_PROG_VER2_2G_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER2_5G_LOW_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER2_5G_MID_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER2_5G_HIGH_OFDM_TXCEIL_BKOFF as i8,
                    ),
                    ft_prog_ver::FT_PROG_VER3 => (
                        FT_PROG_VER3_2G_DSSS_TXCEIL_BKOFF as i8,
                        FT_PROG_VER3_2G_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER3_5G_LOW_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER3_5G_MID_OFDM_TXCEIL_BKOFF as i8,
                        FT_PROG_VER3_5G_HIGH_OFDM_TXCEIL_BKOFF as i8,
                    ),
                }
            } else {
                (0, 0, 0, 0, 0)
            }
        };

        phy_rf_params.max_pwr_ceil.max_dsss_pwr = min(
            tx_pwr_ceil_params.max_pwr_2g_dsss as i8,
            phy_rf_params.max_pwr_ceil.max_dsss_pwr,
        ) - backoff_2g_dsss;

        phy_rf_params.max_pwr_ceil.max_lb_mcs7_pwr = min(
            tx_pwr_ceil_params.max_pwr_2g_mcs7 as i8,
            phy_rf_params.max_pwr_ceil.max_lb_mcs7_pwr,
        ) - backoff_2g_ofdm;

        phy_rf_params.max_pwr_ceil.max_lb_mcs0_pwr = min(
            tx_pwr_ceil_params.max_pwr_2g_mcs0 as i8,
            phy_rf_params.max_pwr_ceil.max_lb_mcs0_pwr,
        ) - backoff_2g_ofdm;

        phy_rf_params.max_pwr_ceil.max_hb_low_chan_mcs7_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_low_mcs7 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_low_chan_mcs7_pwr,
        ) - backoff_5g_lowband;
        phy_rf_params.max_pwr_ceil.max_hb_mid_chan_mcs7_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_mid_mcs7 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_mid_chan_mcs7_pwr,
        ) - backoff_5g_midband;
        phy_rf_params.max_pwr_ceil.max_hb_high_chan_mcs7_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_high_mcs7 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_high_chan_mcs7_pwr,
        ) - backoff_5g_highband;
        phy_rf_params.max_pwr_ceil.max_hb_low_chan_mcs0_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_low_mcs0 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_low_chan_mcs0_pwr,
        ) - backoff_5g_lowband;
        phy_rf_params.max_pwr_ceil.max_hb_mid_chan_mcs0_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_mid_mcs0 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_mid_chan_mcs0_pwr,
        ) - backoff_5g_midband;
        phy_rf_params.max_pwr_ceil.max_hb_high_chan_mcs0_pwr = min(
            tx_pwr_ceil_params.max_pwr_5g_high_mcs0 as i8,
            phy_rf_params.max_pwr_ceil.max_hb_high_chan_mcs0_pwr,
        ) - backoff_5g_highband;

        phy_rf_params
    }
}

pub const RF_PARAM_LENGTH: usize = 34;

#[derive(Debug, Clone, Copy)]
pub struct BoundedU8<const MIN: u8, const MAX: u8> {
    value: u8,
}

impl<const MIN: u8, const MAX: u8> BoundedU8<MIN, MAX> {
    pub const fn get(self) -> u8 {
        self.value
    }
}

impl<const MIN: u8, const MAX: u8> Default for BoundedU8<MIN, MAX> {
    fn default() -> Self {
        Self { value: MIN }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RfParameters {
    // PCB loss for 2.4 GHz band
    pub pcb_loss_2g: BoundedU8<0, 4>,

    // PCB loss for 5 GHz band (5150 MHz - 5350 MHz, Channel-32 - Channel-68)
    pub pcb_loss_5g_band1: BoundedU8<0, 4>,

    // PCB loss for 5 GHz band (5470 MHz - 5730 MHz, Channel-96 - Channel-144)
    pub pcb_loss_5g_band2: BoundedU8<0, 4>,

    // PCB loss for 5 GHz band (5730 MHz - 5895 MHz, Channel-149 - Channel-177)
    pub pcb_loss_5g_band3: BoundedU8<0, 4>,

    /// Antenna gain for 2.4 GHz band
    pub ant_gain_2g: BoundedU8<0, 6>,

    /// Antenna gain for 5 GHz band (5150 MHz - 5350 MHz)
    pub ant_gain_5g_band1: BoundedU8<0, 6>,

    /// Antenna gain for 5 GHz band (5470 MHz - 5730 MHz)
    pub ant_gain_5g_band2: BoundedU8<0, 6>,

    /// Antenna gain for 5 GHz band (5730 MHz - 5895 MHz)
    pub ant_gain_5g_band3: BoundedU8<0, 6>,

    /// DSSS Transmit power backoff (in dB) for lower edge of 2.4 GHz frequency band
    pub band_2g_lower_edge_backoff_dsss: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of 2.4 GHz frequency band
    pub band_2g_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of 2.4 GHz frequency band
    pub band_2g_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// DSSS Transmit power backoff (in dB) for upper edge of 2.4 GHz frequency band
    pub band_2g_upper_edge_backoff_dsss: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of 2.4 GHz frequency band
    pub band_2g_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of 2.4 GHz frequency band
    pub band_2g_upper_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of UNII-1 frequency band
    pub band_unii_1_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of UNII-1 frequency band
    pub band_unii_1_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of UNII-1 frequency band
    pub band_unii_1_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of UNII-1 frequency band
    pub band_unii_1_upper_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of UNII-2A frequency band
    pub band_unii_2a_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of UNII-2A frequency band
    pub band_unii_2a_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of UNII-2A frequency band
    pub band_unii_2a_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of UNII-2A frequency band
    pub band_unii_2a_upper_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of UNII-2C frequency band
    pub band_unii_2c_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of UNII-2C frequency band
    pub band_unii_2c_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of UNII-2C frequency band
    pub band_unii_2c_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of UNII-2C frequency band
    pub band_unii_2c_upper_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of UNII-3 frequency band
    pub band_unii_3_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of UNII-3 frequency band
    pub band_unii_3_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of UNII-3 frequency band
    pub band_unii_3_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of UNII-3 frequency band
    pub band_unii_3_upper_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for lower edge of UNII-4 frequency band
    pub band_unii_4_lower_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for lower edge of UNII-4 frequency band
    pub band_unii_4_lower_edge_backoff_he: BoundedU8<0, 10>,

    /// HT/VHT Transmit power backoff (in dB) for upper edge of UNII-4 frequency band
    pub band_unii_4_upper_edge_backoff_ht: BoundedU8<0, 10>,

    /// HE Transmit power backoff (in dB) for upper edge of UNII-4 frequency band
    pub band_unii_4_upper_edge_backoff_he: BoundedU8<0, 10>,
}

impl RfParameters {
    pub fn populate_slice(&self, out: &mut [::core::ffi::c_uchar]) {
        assert!(
            out.len() >= RF_PARAM_LENGTH,
            "Output slice must have at least {} elements",
            RF_PARAM_LENGTH,
        );

        out[0] = self.band_2g_lower_edge_backoff_dsss.get();
        out[1] = self.band_2g_lower_edge_backoff_ht.get();
        out[2] = self.band_2g_lower_edge_backoff_he.get();
        out[3] = self.band_2g_upper_edge_backoff_dsss.get();
        out[4] = self.band_2g_upper_edge_backoff_ht.get();
        out[5] = self.band_2g_upper_edge_backoff_he.get();
        out[6] = self.band_unii_1_lower_edge_backoff_ht.get();
        out[7] = self.band_unii_1_lower_edge_backoff_he.get();
        out[8] = self.band_unii_1_upper_edge_backoff_ht.get();
        out[9] = self.band_unii_1_upper_edge_backoff_he.get();
        out[10] = self.band_unii_2a_lower_edge_backoff_ht.get();
        out[11] = self.band_unii_2a_lower_edge_backoff_he.get();
        out[12] = self.band_unii_2a_upper_edge_backoff_ht.get();
        out[13] = self.band_unii_2a_upper_edge_backoff_he.get();
        out[14] = self.band_unii_2c_lower_edge_backoff_ht.get();
        out[15] = self.band_unii_2c_lower_edge_backoff_he.get();
        out[16] = self.band_unii_2c_upper_edge_backoff_ht.get();
        out[17] = self.band_unii_2c_upper_edge_backoff_he.get();
        out[18] = self.band_unii_3_lower_edge_backoff_ht.get();
        out[19] = self.band_unii_3_lower_edge_backoff_he.get();
        out[20] = self.band_unii_3_upper_edge_backoff_ht.get();
        out[21] = self.band_unii_3_upper_edge_backoff_he.get();
        out[22] = self.band_unii_4_lower_edge_backoff_ht.get();
        out[23] = self.band_unii_4_lower_edge_backoff_he.get();
        out[24] = self.band_unii_4_upper_edge_backoff_ht.get();
        out[25] = self.band_unii_4_upper_edge_backoff_he.get();
        out[26] = self.ant_gain_2g.get();
        out[27] = self.ant_gain_5g_band1.get();
        out[28] = self.ant_gain_5g_band2.get();
        out[29] = self.ant_gain_5g_band3.get();
        out[30] = self.pcb_loss_2g.get();
        out[31] = self.pcb_loss_5g_band1.get();
        out[32] = self.pcb_loss_5g_band2.get();
        out[33] = self.pcb_loss_5g_band3.get();
    }
}
