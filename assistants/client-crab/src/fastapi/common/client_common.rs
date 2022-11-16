use client_common_traits::ClientCommon;

use crate::client::CrabClient;

impl ClientCommon for CrabClient {
    const CHAIN: &'static str = "crab";

    type Chain = bp_crab::Crab;
}
