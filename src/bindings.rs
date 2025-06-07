#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "defmt")]
use defmt::Formatter;

include!("../thirdparty/bindings.rs");

impl TryFrom<u32> for nrf_wifi_host_rpu_msg_type {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM),
            1 => Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_SUPPLICANT),
            2 => Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_DATA),
            3 => Ok(nrf_wifi_host_rpu_msg_type::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC),
            _ => Err(value),
        }
    }
}

impl TryFrom<u32> for nrf_wifi_sys_events {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_PWR_DATA),
            1 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_INIT_DONE),
            2 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_STATS),
            3 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_DEINIT_DONE),
            4 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_RF_TEST),
            5 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_COEX_CONFIG),
            6 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_INT_UMAC_STATS),
            7 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_RADIOCMD_STATUS),
            8 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_CHANNEL_SET_DONE),
            9 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_MODE_SET_DONE),
            10 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_FILTER_SET_DONE),
            11 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_RAW_TX_DONE),
            12 => Ok(nrf_wifi_sys_events::NRF_WIFI_EVENT_OFFLOADED_RAWTX_STATUS),
            _ => Err(value),
        }
    }
}

impl TryFrom<u32> for nrf_wifi_umac_events {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            256 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNSPECIFIED),
            257 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TRIGGER_SCAN_START),
            258 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_ABORTED),
            259 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_DONE),
            260 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_RESULT),
            261 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_AUTHENTICATE),
            262 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_ASSOCIATE),
            263 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CONNECT),
            264 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DEAUTHENTICATE),
            265 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DISASSOCIATE),
            266 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_STATION),
            267 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DEL_STATION),
            268 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_STATION),
            269 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REMAIN_ON_CHANNEL),
            270 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CANCEL_REMAIN_ON_CHANNEL),
            271 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DISCONNECT),
            272 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_FRAME),
            273 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_COOKIE_RESP),
            274 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_FRAME_TX_STATUS),
            275 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_IFFLAGS_STATUS),
            276 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_TX_POWER),
            277 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_CHANNEL),
            278 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SET_INTERFACE),
            279 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNPROT_DEAUTHENTICATE),
            280 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNPROT_DISASSOCIATE),
            281 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_INTERFACE),
            282 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_WIPHY),
            283 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_IFHWADDR),
            284 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_REG),
            285 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SET_REG),
            286 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REQ_SET_REG),
            287 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_KEY),
            288 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_BEACON_HINT),
            289 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REG_CHANGE),
            290 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_WIPHY_REG_CHANGE),
            291 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_DISPLAY_RESULT),
            292 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CMD_STATUS),
            293 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_BSS_INFO),
            294 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CONFIG_TWT),
            295 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TEARDOWN_TWT),
            296 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TWT_SLEEP),
            297 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_COALESCING),
            298 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_MCAST_FILTER),
            299 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_CONNECTION_INFO),
            300 => Ok(nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_POWER_SAVE_INFO),
            _ => Err(value),
        }
    }
}

impl TryFrom<u32> for nrf_wifi_umac_data_commands {
    type Error = u32;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_MGMT_BUFF_CONFIG),
            1 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_TX_BUFF),
            2 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_TX_BUFF_DONE),
            3 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_RX_BUFF),
            4 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_ON),
            5 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_OFF),
            6 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_PM_MODE),
            7 => Ok(nrf_wifi_umac_data_commands::NRF_WIFI_CMD_PS_GET_FRAMES),
            other => Err(other),
        }
    }
}

impl TryFrom<u32> for nrf70_image_ids {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(nrf70_image_ids::NRF70_IMAGE_UMAC_PRI),
            1 => Ok(nrf70_image_ids::NRF70_IMAGE_UMAC_SEC),
            2 => Ok(nrf70_image_ids::NRF70_IMAGE_LMAC_PRI),
            3 => Ok(nrf70_image_ids::NRF70_IMAGE_LMAC_SEC),
            _ => Err(value),
        }
    }
}

impl TryFrom<u32> for nrf70_feature_flags {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(nrf70_feature_flags::NRF70_FEAT_SYSTEM_MODE),
            2 => Ok(nrf70_feature_flags::NRF70_FEAT_RADIO_TEST),
            4 => Ok(nrf70_feature_flags::NRF70_FEAT_SCAN_ONLY),
            8 => Ok(nrf70_feature_flags::NRF70_FEAT_SYSTEM_WITH_RAW_MODES),
            16 => Ok(nrf70_feature_flags::NRF70_FEAT_OFFLOADED_RAW_TX),
            _ => Err(value),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf70_image_ids {
    fn format(&self, fmt: Formatter) {
        match self {
            nrf70_image_ids::NRF70_IMAGE_UMAC_PRI => defmt::write!(fmt, "UMAC PRIMARY"),
            nrf70_image_ids::NRF70_IMAGE_UMAC_SEC => defmt::write!(fmt, "UMAC SECONDARY"),
            nrf70_image_ids::NRF70_IMAGE_LMAC_PRI => defmt::write!(fmt, "LMAC PRIMARY"),
            nrf70_image_ids::NRF70_IMAGE_LMAC_SEC => defmt::write!(fmt, "LMAC SECONDARY"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_sys_events {
    fn format(&self, fmt: Formatter) {
        match self {
            Self::NRF_WIFI_EVENT_PWR_DATA => defmt::write!(fmt, "NRF_WIFI_EVENT_PWR_DATA"),
            Self::NRF_WIFI_EVENT_INIT_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_INIT_DONE"),
            Self::NRF_WIFI_EVENT_STATS => defmt::write!(fmt, "NRF_WIFI_EVENT_STATS"),
            Self::NRF_WIFI_EVENT_DEINIT_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_DEINIT_DONE"),
            Self::NRF_WIFI_EVENT_RF_TEST => defmt::write!(fmt, "NRF_WIFI_EVENT_RF_TEST"),
            Self::NRF_WIFI_EVENT_COEX_CONFIG => defmt::write!(fmt, "NRF_WIFI_EVENT_COEX_CONFIG"),
            Self::NRF_WIFI_EVENT_INT_UMAC_STATS => defmt::write!(fmt, "NRF_WIFI_EVENT_INT_UMAC_STATS"),
            Self::NRF_WIFI_EVENT_RADIOCMD_STATUS => defmt::write!(fmt, "NRF_WIFI_EVENT_RADIOCMD_STATUS"),
            Self::NRF_WIFI_EVENT_CHANNEL_SET_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_CHANNEL_SET_DONE"),
            Self::NRF_WIFI_EVENT_MODE_SET_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_MODE_SET_DONE"),
            Self::NRF_WIFI_EVENT_FILTER_SET_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_FILTER_SET_DONE"),
            Self::NRF_WIFI_EVENT_RAW_TX_DONE => defmt::write!(fmt, "NRF_WIFI_EVENT_RAW_TX_DONE"),
            Self::NRF_WIFI_EVENT_OFFLOADED_RAWTX_STATUS => defmt::write!(fmt, "NRF_WIFI_EVENT_OFFLOADED_RAWTX_STATUS"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_host_rpu_msg_type {
    fn format(&self, fmt: Formatter) {
        match self {
            Self::NRF_WIFI_HOST_RPU_MSG_TYPE_SYSTEM => defmt::write!(fmt, "SYSTEM"),
            Self::NRF_WIFI_HOST_RPU_MSG_TYPE_SUPPLICANT => defmt::write!(fmt, "SUPPLICANT"),
            Self::NRF_WIFI_HOST_RPU_MSG_TYPE_DATA => defmt::write!(fmt, "DATA"),
            Self::NRF_WIFI_HOST_RPU_MSG_TYPE_UMAC => defmt::write!(fmt, "UMAC"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_umac_events {
    fn format(&self, fmt: Formatter) {
        match self {
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNSPECIFIED => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_UNSPECIFIED")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TRIGGER_SCAN_START => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_TRIGGER_SCAN_START")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_ABORTED => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SCAN_ABORTED")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_DONE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SCAN_DONE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_RESULT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SCAN_RESULT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_AUTHENTICATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_AUTHENTICATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_ASSOCIATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_ASSOCIATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CONNECT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_CONNECT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DEAUTHENTICATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_DEAUTHENTICATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DISASSOCIATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_DISASSOCIATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_NEW_STATION")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DEL_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_DEL_STATION")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_STATION")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REMAIN_ON_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_REMAIN_ON_CHANNEL")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CANCEL_REMAIN_ON_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_CANCEL_REMAIN_ON_CHANNEL")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_DISCONNECT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_DISCONNECT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_FRAME => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_FRAME")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_COOKIE_RESP => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_COOKIE_RESP")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_FRAME_TX_STATUS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_FRAME_TX_STATUS")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_IFFLAGS_STATUS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_IFFLAGS_STATUS")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_TX_POWER => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_TX_POWER")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_CHANNEL")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SET_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SET_INTERFACE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNPROT_DEAUTHENTICATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_UNPROT_DEAUTHENTICATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_UNPROT_DISASSOCIATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_UNPROT_DISASSOCIATE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_NEW_INTERFACE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_NEW_WIPHY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_NEW_WIPHY")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_IFHWADDR => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_IFHWADDR")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_REG")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SET_REG")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REQ_SET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_REQ_SET_REG")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_KEY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_KEY")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_BEACON_HINT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_BEACON_HINT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_REG_CHANGE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_REG_CHANGE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_WIPHY_REG_CHANGE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_WIPHY_REG_CHANGE")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_SCAN_DISPLAY_RESULT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_SCAN_DISPLAY_RESULT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CMD_STATUS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_CMD_STATUS")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_BSS_INFO => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_BSS_INFO")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_CONFIG_TWT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_CONFIG_TWT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TEARDOWN_TWT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_TEARDOWN_TWT")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_TWT_SLEEP => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_TWT_SLEEP")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_COALESCING => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_COALESCING")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_MCAST_FILTER => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_MCAST_FILTER")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_CONNECTION_INFO => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_CONNECTION_INFO")
            }
            nrf_wifi_umac_events::NRF_WIFI_UMAC_EVENT_GET_POWER_SAVE_INFO => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_EVENT_GET_POWER_SAVE_INFO")
            }
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_umac_data_commands {
    fn format(&self, fmt: Formatter) {
        match self {
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_MGMT_BUFF_CONFIG => {
                defmt::write!(fmt, "NRF_WIFI_CMD_MGMT_BUFF_CONFIG")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_TX_BUFF => {
                defmt::write!(fmt, "NRF_WIFI_CMD_TX_BUFF")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_TX_BUFF_DONE => {
                defmt::write!(fmt, "NRF_WIFI_CMD_TX_BUFF_DONE")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_RX_BUFF => {
                defmt::write!(fmt, "NRF_WIFI_CMD_RX_BUFF")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_ON => {
                defmt::write!(fmt, "NRF_WIFI_CMD_CARRIER_ON")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_CARRIER_OFF => {
                defmt::write!(fmt, "NRF_WIFI_CMD_CARRIER_OFF")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_PM_MODE => {
                defmt::write!(fmt, "NRF_WIFI_CMD_PM_MODE")
            }
            nrf_wifi_umac_data_commands::NRF_WIFI_CMD_PS_GET_FRAMES => {
                defmt::write!(fmt, "NRF_WIFI_CMD_PS_GET_FRAMES")
            }
        }
    }
}

impl TryFrom<u32> for nrf_wifi_umac_commands {
    type Error = u32;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_TRIGGER_SCAN),
            1 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_SCAN_RESULTS),
            2 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_AUTHENTICATE),
            3 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_ASSOCIATE),
            4 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEAUTHENTICATE),
            5 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_WIPHY),
            6 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_KEY),
            7 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_KEY),
            8 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_KEY),
            9 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_KEY),
            10 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_BEACON),
            11 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_BEACON),
            12 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_BSS),
            13 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_START_AP),
            14 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_STOP_AP),
            15 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_INTERFACE),
            16 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_INTERFACE),
            17 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_INTERFACE),
            18 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_IFFLAGS),
            19 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_STATION),
            20 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_STATION),
            21 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_STATION),
            22 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_STATION),
            23 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_START_P2P_DEVICE),
            24 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_STOP_P2P_DEVICE),
            25 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REMAIN_ON_CHANNEL),
            26 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CANCEL_REMAIN_ON_CHANNEL),
            27 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_CHANNEL),
            28 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_RADAR_DETECT),
            29 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REGISTER_FRAME),
            30 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_FRAME),
            31 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_JOIN_IBSS),
            32 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_WIN_STA_CONNECT),
            33 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_POWER_SAVE),
            34 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_WOWLAN),
            35 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SUSPEND),
            36 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_RESUME),
            37 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_QOS_MAP),
            38 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_CHANNEL),
            39 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_TX_POWER),
            40 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_INTERFACE),
            41 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_WIPHY),
            42 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_IFHWADDR),
            43 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_IFHWADDR),
            44 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_REG),
            45 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_REG),
            46 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REQ_SET_REG),
            47 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_UAPSD),
            48 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_TWT),
            49 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_TEARDOWN_TWT),
            50 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_ABORT_SCAN),
            51 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_MCAST_FILTER),
            52 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CHANGE_MACADDR),
            53 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_POWER_SAVE_TIMEOUT),
            54 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_CONNECTION_INFO),
            55 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_POWER_SAVE_INFO),
            56 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_LISTEN_INTERVAL),
            57 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_EXTENDED_PS),
            58 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_QUIET_PERIOD),
            59 => Ok(nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_PS_EXIT_STRATEGY),
            other => Err(other),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_umac_commands {
    fn format(&self, fmt: Formatter) {
        match self {
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_TRIGGER_SCAN => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_TRIGGER_SCAN")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_SCAN_RESULTS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_SCAN_RESULTS")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_AUTHENTICATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_AUTHENTICATE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_ASSOCIATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_ASSOCIATE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEAUTHENTICATE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_DEAUTHENTICATE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_WIPHY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_WIPHY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_KEY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_NEW_KEY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_KEY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_DEL_KEY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_KEY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_KEY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_KEY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_KEY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_BEACON => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_NEW_BEACON")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_BEACON => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_BEACON")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_BSS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_BSS")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_START_AP => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_START_AP")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_STOP_AP => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_STOP_AP")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_NEW_INTERFACE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_INTERFACE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_DEL_INTERFACE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_IFFLAGS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_IFFLAGS")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_NEW_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_NEW_STATION")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_DEL_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_DEL_STATION")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_STATION")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_STATION => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_STATION")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_START_P2P_DEVICE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_START_P2P_DEVICE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_STOP_P2P_DEVICE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_STOP_P2P_DEVICE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REMAIN_ON_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_REMAIN_ON_CHANNEL")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CANCEL_REMAIN_ON_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CANCEL_REMAIN_ON_CHANNEL")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_CHANNEL")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_RADAR_DETECT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_RADAR_DETECT")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REGISTER_FRAME => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_REGISTER_FRAME")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_FRAME => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_FRAME")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_JOIN_IBSS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_JOIN_IBSS")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_WIN_STA_CONNECT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_WIN_STA_CONNECT")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_POWER_SAVE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_POWER_SAVE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_WOWLAN => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_WOWLAN")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SUSPEND => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SUSPEND")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_RESUME => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_RESUME")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_QOS_MAP => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_QOS_MAP")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_CHANNEL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_CHANNEL")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_TX_POWER => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_TX_POWER")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_INTERFACE => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_INTERFACE")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_WIPHY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_WIPHY")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_IFHWADDR => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_IFHWADDR")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_IFHWADDR => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_IFHWADDR")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_REG")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_REG")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_REQ_SET_REG => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_REQ_SET_REG")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_UAPSD => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CONFIG_UAPSD")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_TWT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CONFIG_TWT")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_TEARDOWN_TWT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_TEARDOWN_TWT")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_ABORT_SCAN => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_ABORT_SCAN")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_MCAST_FILTER => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_MCAST_FILTER")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CHANGE_MACADDR => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CHANGE_MACADDR")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_POWER_SAVE_TIMEOUT => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_POWER_SAVE_TIMEOUT")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_CONNECTION_INFO => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_CONNECTION_INFO")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_GET_POWER_SAVE_INFO => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_GET_POWER_SAVE_INFO")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_SET_LISTEN_INTERVAL => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_SET_LISTEN_INTERVAL")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_EXTENDED_PS => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CONFIG_EXTENDED_PS")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_CONFIG_QUIET_PERIOD => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_CONFIG_QUIET_PERIOD")
            }
            nrf_wifi_umac_commands::NRF_WIFI_UMAC_CMD_PS_EXIT_STRATEGY => {
                defmt::write!(fmt, "NRF_WIFI_UMAC_CMD_PS_EXIT_STRATEGY")
            }
        }
    }
}

impl TryFrom<u32> for nrf_wifi_rx_pkt_type {
    type Error = u32;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(nrf_wifi_rx_pkt_type::NRF_WIFI_RX_PKT_DATA),
            1 => Ok(nrf_wifi_rx_pkt_type::NRF_WIFI_RX_PKT_BCN_PRB_RSP),
            2 => Ok(nrf_wifi_rx_pkt_type::NRF_WIFI_RAW_RX_PKT),
            other => Err(other),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_sys_head {
    fn format(&self, fmt: Formatter) {
        let cmd_event = self.cmd_event;
        let len = self.len;

        defmt::write!(fmt, "nrf_wifi_sys_head {{ cmd_event: {}, len: {} }}", cmd_event, len)
    }
}

#[cfg(feature = "defmt")]
impl Format for rpu_phy_stats {
    fn format(&self, fmt: Formatter) {
        let rssi_avg = self.rssi_avg;
        let pdout_val = self.pdout_val;
        let ofdm_crc32_pass_cnt = self.ofdm_crc32_pass_cnt;
        let ofdm_crc32_fail_cnt = self.ofdm_crc32_fail_cnt;
        let dsss_crc32_pass_cnt = self.dsss_crc32_pass_cnt;
        let dsss_crc32_fail_cnt = self.dsss_crc32_fail_cnt;

        defmt::write!(
            fmt,
            "rpu_phy_stats {{ \
                rssi_avg: {}, \
                pdout_val: {}, \
                ofdm_crc32_pass_cnt: {}, \
                ofdm_crc32_fail_cnt: {}, \
                dsss_crc32_pass_cnt: {}, \
                dsss_crc32_fail_cnt: {} \
            }}",
            rssi_avg,
            pdout_val,
            ofdm_crc32_pass_cnt,
            ofdm_crc32_fail_cnt,
            dsss_crc32_pass_cnt,
            dsss_crc32_fail_cnt,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for rpu_lmac_stats {
    fn format(&self, fmt: Formatter) {
        let reset_cmd_cnt = self.reset_cmd_cnt;
        let reset_complete_event_cnt = self.reset_complete_event_cnt;
        let unable_gen_event = self.unable_gen_event;
        let ch_prog_cmd_cnt = self.ch_prog_cmd_cnt;
        let channel_prog_done = self.channel_prog_done;
        let tx_pkt_cnt = self.tx_pkt_cnt;
        let tx_pkt_done_cnt = self.tx_pkt_done_cnt;
        let scan_pkt_cnt = self.scan_pkt_cnt;
        let internal_pkt_cnt = self.internal_pkt_cnt;
        let internal_pkt_done_cnt = self.internal_pkt_done_cnt;
        let ack_resp_cnt = self.ack_resp_cnt;
        let tx_timeout = self.tx_timeout;
        let deagg_isr = self.deagg_isr;
        let deagg_inptr_desc_empty = self.deagg_inptr_desc_empty;
        let deagg_circular_buffer_full = self.deagg_circular_buffer_full;
        let lmac_rxisr_cnt = self.lmac_rxisr_cnt;
        let rx_decryptcnt = self.rx_decryptcnt;
        let process_decrypt_fail = self.process_decrypt_fail;
        let prepa_rx_event_fail = self.prepa_rx_event_fail;
        let rx_core_pool_full_cnt = self.rx_core_pool_full_cnt;
        let rx_mpdu_crc_success_cnt = self.rx_mpdu_crc_success_cnt;
        let rx_mpdu_crc_fail_cnt = self.rx_mpdu_crc_fail_cnt;
        let rx_ofdm_crc_success_cnt = self.rx_ofdm_crc_success_cnt;
        let rx_ofdm_crc_fail_cnt = self.rx_ofdm_crc_fail_cnt;
        let rxDSSSCrcSuccessCnt = self.rxDSSSCrcSuccessCnt;
        let rxDSSSCrcFailCnt = self.rxDSSSCrcFailCnt;
        let rx_crypto_start_cnt = self.rx_crypto_start_cnt;
        let rx_crypto_done_cnt = self.rx_crypto_done_cnt;
        let rx_event_buf_full = self.rx_event_buf_full;
        let rx_extram_buf_full = self.rx_extram_buf_full;
        let scan_req = self.scan_req;
        let scan_complete = self.scan_complete;
        let scan_abort_req = self.scan_abort_req;
        let scan_abort_complete = self.scan_abort_complete;
        let internal_buf_pool_null = self.internal_buf_pool_null;
        let rpu_hw_lockup_count = self.rpu_hw_lockup_count;
        let rpu_hw_lockup_recovery_done = self.rpu_hw_lockup_recovery_done;

        defmt::write!(
            fmt,
            "rpu_lmac_stats {{ \
 reset_cmd_cnt: {}, reset_complete_event_cnt: {}, unable_gen_event: {}, ch_prog_cmd_cnt: {}, \
 channel_prog_done: {}, tx_pkt_cnt: {}, tx_pkt_done_cnt: {}, scan_pkt_cnt: {}, internal_pkt_cnt: {}, \
 internal_pkt_done_cnt: {}, ack_resp_cnt: {}, tx_timeout: {}, deagg_isr: {}, deagg_inptr_desc_empty: {}, \
 deagg_circular_buffer_full: {}, lmac_rxisr_cnt: {}, rx_decryptcnt: {}, process_decrypt_fail: {}, \
 prepa_rx_event_fail: {}, rx_core_pool_full_cnt: {}, rx_mpdu_crc_success_cnt: {}, rx_mpdu_crc_fail_cnt: {}, \
 rx_ofdm_crc_success_cnt: {}, rx_ofdm_crc_fail_cnt: {}, rxDSSSCrcSuccessCnt: {}, rxDSSSCrcFailCnt: {}, \
 rx_crypto_start_cnt: {}, rx_crypto_done_cnt: {}, rx_event_buf_full: {}, rx_extram_buf_full: {}, \
 scan_req: {}, scan_complete: {}, scan_abort_req: {}, scan_abort_complete: {}, \
 internal_buf_pool_null: {}, rpu_hw_lockup_count: {}, rpu_hw_lockup_recovery_done: {} \
}}",
            reset_cmd_cnt,
            reset_complete_event_cnt,
            unable_gen_event,
            ch_prog_cmd_cnt,
            channel_prog_done,
            tx_pkt_cnt,
            tx_pkt_done_cnt,
            scan_pkt_cnt,
            internal_pkt_cnt,
            internal_pkt_done_cnt,
            ack_resp_cnt,
            tx_timeout,
            deagg_isr,
            deagg_inptr_desc_empty,
            deagg_circular_buffer_full,
            lmac_rxisr_cnt,
            rx_decryptcnt,
            process_decrypt_fail,
            prepa_rx_event_fail,
            rx_core_pool_full_cnt,
            rx_mpdu_crc_success_cnt,
            rx_mpdu_crc_fail_cnt,
            rx_ofdm_crc_success_cnt,
            rx_ofdm_crc_fail_cnt,
            rxDSSSCrcSuccessCnt,
            rxDSSSCrcFailCnt,
            rx_crypto_start_cnt,
            rx_crypto_done_cnt,
            rx_event_buf_full,
            rx_extram_buf_full,
            scan_req,
            scan_complete,
            scan_abort_req,
            scan_abort_complete,
            internal_buf_pool_null,
            rpu_hw_lockup_count,
            rpu_hw_lockup_recovery_done,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for umac_tx_dbg_params {
    fn format(&self, fmt: Formatter) {
        // Bind each packed field to a local variable
        let tx_cmd = self.tx_cmd;
        let tx_non_coalesce_pkts_rcvd_from_host = self.tx_non_coalesce_pkts_rcvd_from_host;
        let tx_coalesce_pkts_rcvd_from_host = self.tx_coalesce_pkts_rcvd_from_host;
        let tx_max_coalesce_pkts_rcvd_from_host = self.tx_max_coalesce_pkts_rcvd_from_host;
        let tx_cmds_max_used = self.tx_cmds_max_used;
        let tx_cmds_currently_in_use = self.tx_cmds_currently_in_use;
        let tx_done_events_send_to_host = self.tx_done_events_send_to_host;
        let tx_done_success_pkts_to_host = self.tx_done_success_pkts_to_host;
        let tx_done_failure_pkts_to_host = self.tx_done_failure_pkts_to_host;
        let tx_cmds_with_crypto_pkts_rcvd_from_host = self.tx_cmds_with_crypto_pkts_rcvd_from_host;
        let tx_cmds_with_non_crypto_pkts_rcvd_from_host = self.tx_cmds_with_non_crypto_pkts_rcvd_from_host;
        let tx_cmds_with_broadcast_pkts_rcvd_from_host = self.tx_cmds_with_broadcast_pkts_rcvd_from_host;
        let tx_cmds_with_multicast_pkts_rcvd_from_host = self.tx_cmds_with_multicast_pkts_rcvd_from_host;
        let tx_cmds_with_unicast_pkts_rcvd_from_host = self.tx_cmds_with_unicast_pkts_rcvd_from_host;
        let xmit = self.xmit;
        let send_addba_req = self.send_addba_req;
        let addba_resp = self.addba_resp;
        let softmac_tx = self.softmac_tx;
        let internal_pkts = self.internal_pkts;
        let external_pkts = self.external_pkts;
        let tx_cmds_to_lmac = self.tx_cmds_to_lmac;
        let tx_dones_from_lmac = self.tx_dones_from_lmac;
        let total_cmds_to_lmac = self.total_cmds_to_lmac;
        let tx_packet_data_count = self.tx_packet_data_count;
        let tx_packet_mgmt_count = self.tx_packet_mgmt_count;
        let tx_packet_beacon_count = self.tx_packet_beacon_count;
        let tx_packet_probe_req_count = self.tx_packet_probe_req_count;
        let tx_packet_auth_count = self.tx_packet_auth_count;
        let tx_packet_deauth_count = self.tx_packet_deauth_count;
        let tx_packet_assoc_req_count = self.tx_packet_assoc_req_count;
        let tx_packet_disassoc_count = self.tx_packet_disassoc_count;
        let tx_packet_action_count = self.tx_packet_action_count;
        let tx_packet_other_mgmt_count = self.tx_packet_other_mgmt_count;
        let tx_packet_non_mgmt_data_count = self.tx_packet_non_mgmt_data_count;

        defmt::write!(
            fmt,
            "umac_tx_dbg_params {{ \
 tx_cmd: {}, tx_non_coalesce_pkts_rcvd_from_host: {}, tx_coalesce_pkts_rcvd_from_host: {}, \
 tx_max_coalesce_pkts_rcvd_from_host: {}, tx_cmds_max_used: {}, tx_cmds_currently_in_use: {}, \
 tx_done_events_send_to_host: {}, tx_done_success_pkts_to_host: {}, tx_done_failure_pkts_to_host: {}, \
 tx_cmds_with_crypto_pkts_rcvd_from_host: {}, tx_cmds_with_non_crypto_pkts_rcvd_from_host: {}, \
 tx_cmds_with_broadcast_pkts_rcvd_from_host: {}, tx_cmds_with_multicast_pkts_rcvd_from_host: {}, \
 tx_cmds_with_unicast_pkts_rcvd_from_host: {}, xmit: {}, send_addba_req: {}, addba_resp: {}, \
 softmac_tx: {}, internal_pkts: {}, external_pkts: {}, tx_cmds_to_lmac: {}, tx_dones_from_lmac: {}, \
 total_cmds_to_lmac: {}, tx_packet_data_count: {}, tx_packet_mgmt_count: {}, tx_packet_beacon_count: {}, \
 tx_packet_probe_req_count: {}, tx_packet_auth_count: {}, tx_packet_deauth_count: {}, \
 tx_packet_assoc_req_count: {}, tx_packet_disassoc_count: {}, tx_packet_action_count: {}, \
 tx_packet_other_mgmt_count: {}, tx_packet_non_mgmt_data_count: {} \
}}",
            tx_cmd,
            tx_non_coalesce_pkts_rcvd_from_host,
            tx_coalesce_pkts_rcvd_from_host,
            tx_max_coalesce_pkts_rcvd_from_host,
            tx_cmds_max_used,
            tx_cmds_currently_in_use,
            tx_done_events_send_to_host,
            tx_done_success_pkts_to_host,
            tx_done_failure_pkts_to_host,
            tx_cmds_with_crypto_pkts_rcvd_from_host,
            tx_cmds_with_non_crypto_pkts_rcvd_from_host,
            tx_cmds_with_broadcast_pkts_rcvd_from_host,
            tx_cmds_with_multicast_pkts_rcvd_from_host,
            tx_cmds_with_unicast_pkts_rcvd_from_host,
            xmit,
            send_addba_req,
            addba_resp,
            softmac_tx,
            internal_pkts,
            external_pkts,
            tx_cmds_to_lmac,
            tx_dones_from_lmac,
            total_cmds_to_lmac,
            tx_packet_data_count,
            tx_packet_mgmt_count,
            tx_packet_beacon_count,
            tx_packet_probe_req_count,
            tx_packet_auth_count,
            tx_packet_deauth_count,
            tx_packet_assoc_req_count,
            tx_packet_disassoc_count,
            tx_packet_action_count,
            tx_packet_other_mgmt_count,
            tx_packet_non_mgmt_data_count,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for umac_rx_dbg_params {
    fn format(&self, fmt: Formatter) {
        let lmac_events = self.lmac_events;
        let rx_events = self.rx_events;
        let rx_coalesce_events = self.rx_coalesce_events;
        let total_rx_pkts_from_lmac = self.total_rx_pkts_from_lmac;
        let max_refill_gap = self.max_refill_gap;
        let current_refill_gap = self.current_refill_gap;
        let out_of_order_mpdus = self.out_of_order_mpdus;
        let reorder_free_mpdus = self.reorder_free_mpdus;
        let umac_consumed_pkts = self.umac_consumed_pkts;
        let host_consumed_pkts = self.host_consumed_pkts;
        let rx_mbox_post = self.rx_mbox_post;
        let rx_mbox_receive = self.rx_mbox_receive;
        let reordering_ampdu = self.reordering_ampdu;
        let timer_mbox_post = self.timer_mbox_post;
        let timer_mbox_rcv = self.timer_mbox_rcv;
        let work_mbox_post = self.work_mbox_post;
        let work_mbox_rcv = self.work_mbox_rcv;
        let tasklet_mbox_post = self.tasklet_mbox_post;
        let tasklet_mbox_rcv = self.tasklet_mbox_rcv;
        let userspace_offload_frames = self.userspace_offload_frames;
        let alloc_buf_fail = self.alloc_buf_fail;
        let rx_packet_total_count = self.rx_packet_total_count;
        let rx_packet_data_count = self.rx_packet_data_count;
        let rx_packet_qos_data_count = self.rx_packet_qos_data_count;
        let rx_packet_protected_data_count = self.rx_packet_protected_data_count;
        let rx_packet_mgmt_count = self.rx_packet_mgmt_count;
        let rx_packet_beacon_count = self.rx_packet_beacon_count;
        let rx_packet_probe_resp_count = self.rx_packet_probe_resp_count;
        let rx_packet_auth_count = self.rx_packet_auth_count;
        let rx_packet_deauth_count = self.rx_packet_deauth_count;
        let rx_packet_assoc_resp_count = self.rx_packet_assoc_resp_count;
        let rx_packet_disassoc_count = self.rx_packet_disassoc_count;
        let rx_packet_action_count = self.rx_packet_action_count;
        let rx_packet_probe_req_count = self.rx_packet_probe_req_count;
        let rx_packet_other_mgmt_count = self.rx_packet_other_mgmt_count;
        let max_coalesce_pkts = self.max_coalesce_pkts;
        let null_skb_pointer_from_lmac = self.null_skb_pointer_from_lmac;
        let unexpected_mgmt_pkt = self.unexpected_mgmt_pkt;

        defmt::write!(
            fmt,
            "umac_rx_dbg_params {{ \
 lmac_events: {}, rx_events: {}, rx_coalesce_events: {}, total_rx_pkts_from_lmac: {}, \
 max_refill_gap: {}, current_refill_gap: {}, out_of_order_mpdus: {}, reorder_free_mpdus: {}, \
 umac_consumed_pkts: {}, host_consumed_pkts: {}, rx_mbox_post: {}, rx_mbox_receive: {}, \
 reordering_ampdu: {}, timer_mbox_post: {}, timer_mbox_rcv: {}, work_mbox_post: {}, \
 work_mbox_rcv: {}, tasklet_mbox_post: {}, tasklet_mbox_rcv: {}, userspace_offload_frames: {}, \
 alloc_buf_fail: {}, rx_packet_total_count: {}, rx_packet_data_count: {}, \
 rx_packet_qos_data_count: {}, rx_packet_protected_data_count: {}, rx_packet_mgmt_count: {}, \
 rx_packet_beacon_count: {}, rx_packet_probe_resp_count: {}, rx_packet_auth_count: {}, \
 rx_packet_deauth_count: {}, rx_packet_assoc_resp_count: {}, rx_packet_disassoc_count: {}, \
 rx_packet_action_count: {}, rx_packet_probe_req_count: {}, rx_packet_other_mgmt_count: {}, \
 max_coalesce_pkts: {}, null_skb_pointer_from_lmac: {}, unexpected_mgmt_pkt: {} \
}}",
            lmac_events,
            rx_events,
            rx_coalesce_events,
            total_rx_pkts_from_lmac,
            max_refill_gap,
            current_refill_gap,
            out_of_order_mpdus,
            reorder_free_mpdus,
            umac_consumed_pkts,
            host_consumed_pkts,
            rx_mbox_post,
            rx_mbox_receive,
            reordering_ampdu,
            timer_mbox_post,
            timer_mbox_rcv,
            work_mbox_post,
            work_mbox_rcv,
            tasklet_mbox_post,
            tasklet_mbox_rcv,
            userspace_offload_frames,
            alloc_buf_fail,
            rx_packet_total_count,
            rx_packet_data_count,
            rx_packet_qos_data_count,
            rx_packet_protected_data_count,
            rx_packet_mgmt_count,
            rx_packet_beacon_count,
            rx_packet_probe_resp_count,
            rx_packet_auth_count,
            rx_packet_deauth_count,
            rx_packet_assoc_resp_count,
            rx_packet_disassoc_count,
            rx_packet_action_count,
            rx_packet_probe_req_count,
            rx_packet_other_mgmt_count,
            max_coalesce_pkts,
            null_skb_pointer_from_lmac,
            unexpected_mgmt_pkt,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for umac_cmd_evnt_dbg_params {
    fn format(&self, fmt: Formatter) {
        // Bind each packed field to a local variable
        let cmd_init = self.cmd_init;
        let event_init_done = self.event_init_done;
        let cmd_rf_test = self.cmd_rf_test;
        let cmd_connect = self.cmd_connect;
        let cmd_get_stats = self.cmd_get_stats;
        let event_ps_state = self.event_ps_state;
        let cmd_set_reg = self.cmd_set_reg;
        let cmd_get_reg = self.cmd_get_reg;
        let cmd_req_set_reg = self.cmd_req_set_reg;
        let cmd_trigger_scan = self.cmd_trigger_scan;
        let event_scan_done = self.event_scan_done;
        let cmd_get_scan = self.cmd_get_scan;
        let umac_scan_req = self.umac_scan_req;
        let umac_scan_complete = self.umac_scan_complete;
        let umac_scan_busy = self.umac_scan_busy;
        let cmd_auth = self.cmd_auth;
        let cmd_assoc = self.cmd_assoc;
        let cmd_deauth = self.cmd_deauth;
        let cmd_register_frame = self.cmd_register_frame;
        let cmd_frame = self.cmd_frame;
        let cmd_del_key = self.cmd_del_key;
        let cmd_new_key = self.cmd_new_key;
        let cmd_set_key = self.cmd_set_key;
        let cmd_get_key = self.cmd_get_key;
        let event_beacon_hint = self.event_beacon_hint;
        let event_reg_change = self.event_reg_change;
        let event_wiphy_reg_change = self.event_wiphy_reg_change;
        let cmd_set_station = self.cmd_set_station;
        let cmd_new_station = self.cmd_new_station;
        let cmd_del_station = self.cmd_del_station;
        let cmd_new_interface = self.cmd_new_interface;
        let cmd_set_interface = self.cmd_set_interface;
        let cmd_get_interface = self.cmd_get_interface;
        let cmd_set_ifflags = self.cmd_set_ifflags;
        let cmd_set_ifflags_done = self.cmd_set_ifflags_done;
        let cmd_set_bss = self.cmd_set_bss;
        let cmd_set_wiphy = self.cmd_set_wiphy;
        let cmd_start_ap = self.cmd_start_ap;
        let lmac_cmd_ps = self.LMAC_CMD_PS;
        let curr_state = self.CURR_STATE;

        defmt::write!(
            fmt,
            "umac_cmd_evnt_dbg_params {{ \
 cmd_init: {}, event_init_done: {}, cmd_rf_test: {}, cmd_connect: {}, cmd_get_stats: {}, \
 event_ps_state: {}, cmd_set_reg: {}, cmd_get_reg: {}, cmd_req_set_reg: {}, \
 cmd_trigger_scan: {}, event_scan_done: {}, cmd_get_scan: {}, umac_scan_req: {}, \
 umac_scan_complete: {}, umac_scan_busy: {}, cmd_auth: {}, cmd_assoc: {}, cmd_deauth: {}, \
 cmd_register_frame: {}, cmd_frame: {}, cmd_del_key: {}, cmd_new_key: {}, cmd_set_key: {}, \
 cmd_get_key: {}, event_beacon_hint: {}, event_reg_change: {}, event_wiphy_reg_change: {}, \
 cmd_set_station: {}, cmd_new_station: {}, cmd_del_station: {}, cmd_new_interface: {}, \
 cmd_set_interface: {}, cmd_get_interface: {}, cmd_set_ifflags: {}, cmd_set_ifflags_done: {}, \
 cmd_set_bss: {}, cmd_set_wiphy: {}, cmd_start_ap: {}, LMAC_CMD_PS: {}, CURR_STATE: {} \
}}",
            cmd_init,
            event_init_done,
            cmd_rf_test,
            cmd_connect,
            cmd_get_stats,
            event_ps_state,
            cmd_set_reg,
            cmd_get_reg,
            cmd_req_set_reg,
            cmd_trigger_scan,
            event_scan_done,
            cmd_get_scan,
            umac_scan_req,
            umac_scan_complete,
            umac_scan_busy,
            cmd_auth,
            cmd_assoc,
            cmd_deauth,
            cmd_register_frame,
            cmd_frame,
            cmd_del_key,
            cmd_new_key,
            cmd_set_key,
            cmd_get_key,
            event_beacon_hint,
            event_reg_change,
            event_wiphy_reg_change,
            cmd_set_station,
            cmd_new_station,
            cmd_del_station,
            cmd_new_interface,
            cmd_set_interface,
            cmd_get_interface,
            cmd_set_ifflags,
            cmd_set_ifflags_done,
            cmd_set_bss,
            cmd_set_wiphy,
            cmd_start_ap,
            lmac_cmd_ps,
            curr_state,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for nrf_wifi_interface_stats {
    fn format(&self, fmt: Formatter) {
        // Bind each packed field to a local variable
        let tx_unicast_pkt_count = self.tx_unicast_pkt_count;
        let tx_multicast_pkt_count = self.tx_multicast_pkt_count;
        let tx_broadcast_pkt_count = self.tx_broadcast_pkt_count;
        let tx_bytes = self.tx_bytes;
        let rx_unicast_pkt_count = self.rx_unicast_pkt_count;
        let rx_multicast_pkt_count = self.rx_multicast_pkt_count;
        let rx_broadcast_pkt_count = self.rx_broadcast_pkt_count;
        let rx_beacon_success_count = self.rx_beacon_success_count;
        let rx_beacon_miss_count = self.rx_beacon_miss_count;
        let rx_bytes = self.rx_bytes;
        let rx_checksum_error_count = self.rx_checksum_error_count;
        let replay_attack_drop_cnt = self.replay_attack_drop_cnt;

        defmt::write!(
            fmt,
            "nrf_wifi_interface_stats {{ \
 tx_unicast_pkt_count: {}, tx_multicast_pkt_count: {}, tx_broadcast_pkt_count: {}, tx_bytes: {}, \
 rx_unicast_pkt_count: {}, rx_multicast_pkt_count: {}, rx_broadcast_pkt_count: {}, \
 rx_beacon_success_count: {}, rx_beacon_miss_count: {}, rx_bytes: {}, \
 rx_checksum_error_count: {}, replay_attack_drop_cnt: {} \
}}",
            tx_unicast_pkt_count,
            tx_multicast_pkt_count,
            tx_broadcast_pkt_count,
            tx_bytes,
            rx_unicast_pkt_count,
            rx_multicast_pkt_count,
            rx_broadcast_pkt_count,
            rx_beacon_success_count,
            rx_beacon_miss_count,
            rx_bytes,
            rx_checksum_error_count,
            replay_attack_drop_cnt,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for rpu_umac_stats {
    fn format(&self, fmt: Formatter) {
        let tx_dbg_params = self.tx_dbg_params;
        let rx_dbg_params = self.rx_dbg_params;
        let cmd_evnt_dbg_params = self.cmd_evnt_dbg_params;
        let interface_data_stats = self.interface_data_stats;

        defmt::write!(
            fmt,
            "rpu_umac_stats {{ \
 tx_dbg_params: {}, \
 rx_dbg_params: {}, \
 cmd_evnt_dbg_params: {}, \
 interface_data_stats: {} \
}}",
            tx_dbg_params,
            rx_dbg_params,
            cmd_evnt_dbg_params,
            interface_data_stats,
        );
    }
}

#[cfg(feature = "defmt")]
impl Format for rpu_sys_fw_stats {
    fn format(&self, fmt: Formatter) {
        // Bind each packed field to a local variable
        let phy = self.phy;
        let lmac = self.lmac;
        let umac = self.umac;

        defmt::write!(
            fmt,
            "rpu_sys_fw_stats {{ \
 phy: {}, \
 lmac: {}, \
 umac: {} \
}}",
            phy,
            lmac,
            umac,
        );
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_sys_umac_event_stats {
    fn format(&self, fmt: Formatter) {
        defmt::write!(
            fmt,
            "nrf_wifi_sys_umac_event_stats {{  \
                                sys_head: {} \
                                fw: {},
            }}",
            self.sys_head,
            self.fw,
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for nrf_wifi_umac_hdr {
    fn format(&self, fmt: Formatter) {
        let portid = self.portid;
        let seq = self.seq;
        let cmd_evnt = self.cmd_evnt;
        let rpu_ret_val = self.rpu_ret_val;
        let ids = self.ids;

        defmt::write!(
            fmt,
            "nrf_wifi_umac_hdr {{ \
 portid: {}, seq: {}, cmd_evnt: {}, rpu_ret_val: {}, ids: {} \
}}",
            portid,
            seq,
            cmd_evnt,
            rpu_ret_val,
            ids,
        );
    }
}

impl defmt::Format for nrf_wifi_index_ids {
    fn format(&self, fmt: Formatter) {
        let valid_fields = self.valid_fields;
        let ifaceindex = self.ifaceindex;
        let nrf_wifi_wiphy_idx = self.nrf_wifi_wiphy_idx;
        let wdev_id = self.wdev_id;

        defmt::write!(
            fmt,
            "nrf_wifi_index_ids {{ \
 valid_fields: {}, ifaceindex: {}, nrf_wifi_wiphy_idx: {}, wdev_id: {} \
}}",
            valid_fields,
            ifaceindex,
            nrf_wifi_wiphy_idx,
            wdev_id,
        );
    }
}
