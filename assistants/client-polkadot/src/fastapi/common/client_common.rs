use client_common_traits::ClientCommon;

use crate::client::PolkadotClient;

impl ClientCommon for PolkadotClient {
    const CHAIN: &'static str = "polkadot";

    type Chain = bp_polkadot_core::PolkadotLike;
}
