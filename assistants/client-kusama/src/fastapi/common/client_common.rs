use client_common_traits::ClientCommon;

use crate::client::KusamaClient;

impl ClientCommon for KusamaClient {
    const CHAIN: &'static str = "kusama";

    type Chain = bp_kusama::Kusama;
}
