use client_common_traits::ClientCommon;

use crate::client::PangolinParachainClient;

impl ClientCommon for PangolinParachainClient {
    const CHAIN: &'static str = "pangolinparachain";

    type Chain = bp_pangolin_parachain::PangolinParachain;
}
