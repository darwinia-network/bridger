use client_common_traits::ClientCommon;

use crate::client::DarwiniaClient;

impl ClientCommon for DarwiniaClient {
    const CHAIN: &'static str = "darwinia";

    type Chain = bp_darwinia::Darwinia;
}
