use client_common_traits::ClientCommon;

use crate::client::PangolinClient;

impl ClientCommon for PangolinClient {
    const CHAIN: &'static str = "pangolin";

    type Chain = bp_darwinia_core::DarwiniaLike;
}
