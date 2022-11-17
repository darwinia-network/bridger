use client_common_traits::ClientCommon;

use crate::client::MoonbaseClient;

impl ClientCommon for MoonbaseClient {
    const CHAIN: &'static str = "moonbase";

    type Chain = bp_kusama::Kusama;
}
