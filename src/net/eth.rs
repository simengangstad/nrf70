use crate::bindings::{
    nrf_wifi_fmac_eth_hdr, nrf_wifi_fmac_ieee80211_hdr, NRF_WIFI_FCTL_FROMDS, NRF_WIFI_FCTL_TODS,
    NRF_WIFI_FMAC_ETH_P_802_3_MIN, NRF_WIFI_FMAC_ETH_P_AARP, NRF_WIFI_FMAC_ETH_P_IPX,
};

pub fn get_type(buffer: &[u8; 2]) -> u16 {
    ((buffer[0] as u16) << 8) | (buffer[1] as u16)
}

pub fn get_skip_header_bytes(eth_type: u16) -> usize {
    // Ethernet-II snap header (RFC1042 for most EtherTypes)
    static LLC_HEADER: [u8; 6] = [0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00];

    // Bridge-Tunnel header (for EtherTypes ETH_P_AARP and ETH_P_IPX)
    static AARP_IPX_HEADER: [u8; 6] = [0xaa, 0xaa, 0x03, 0x00, 0x00, 0xf8];

    let mut skip_header_bytes = size_of_val(&eth_type);

    if eth_type == NRF_WIFI_FMAC_ETH_P_AARP as u16 || eth_type == NRF_WIFI_FMAC_ETH_P_IPX as u16 {
        skip_header_bytes += size_of_val(&AARP_IPX_HEADER);
    } else if eth_type >= NRF_WIFI_FMAC_ETH_P_802_3_MIN as u16 {
        skip_header_bytes += size_of_val(&LLC_HEADER);
    }

    return skip_header_bytes;
}

impl nrf_wifi_fmac_eth_hdr {
    pub fn new(data_size: u16, header: &nrf_wifi_fmac_ieee80211_hdr, eth_type: u16) -> nrf_wifi_fmac_eth_hdr {
        let proto: u16 = {
            if eth_type >= NRF_WIFI_FMAC_ETH_P_802_3_MIN as u16 {
                (eth_type >> 8) | (eth_type << 8)
            } else {
                data_size
            }
        };

        let flags = (header.fc as u32) & (NRF_WIFI_FCTL_TODS | NRF_WIFI_FCTL_FROMDS);

        if flags == (NRF_WIFI_FCTL_TODS | NRF_WIFI_FCTL_FROMDS) {
            return nrf_wifi_fmac_eth_hdr {
                src: header.addr_4,
                dst: header.addr_1,
                proto,
            };
        } else if flags == NRF_WIFI_FCTL_FROMDS {
            return nrf_wifi_fmac_eth_hdr {
                src: header.addr_3,
                dst: header.addr_1,
                proto,
            };
        } else if flags == NRF_WIFI_FCTL_TODS {
            return nrf_wifi_fmac_eth_hdr {
                src: header.addr_2,
                dst: header.addr_3,
                proto,
            };
        }

        return nrf_wifi_fmac_eth_hdr {
            src: header.addr_2,
            dst: header.addr_1,
            proto,
        };
    }
}
