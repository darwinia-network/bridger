use client_common_traits::ClientCommon;

use crate::client::CrabParachainClient;

impl ClientCommon for CrabParachainClient {
    const CHAIN: &'static str = "crabparachain";

    type Chain = bp_crab_parachain::CrabParachain;
}
