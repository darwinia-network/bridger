use client_common_traits::ClientCommon;

use crate::client::PangoroClient;

impl ClientCommon for PangoroClient {
    const CHAIN: &'static str = "pangoro";

    type Chain = bp_darwinia_core::DarwiniaLike;
}
