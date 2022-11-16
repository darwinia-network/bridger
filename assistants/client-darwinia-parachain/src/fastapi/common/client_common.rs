use client_common_traits::ClientCommon;

use crate::client::DarwiniaParachainClient;

impl ClientCommon for DarwiniaParachainClient {
    const CHAIN: &'static str = "darwiniaparachain";

    type Chain = bp_darwinia_parachain::DarwiniaParachain;
}
