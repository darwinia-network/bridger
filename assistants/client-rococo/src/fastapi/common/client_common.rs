use client_common_traits::ClientCommon;

use crate::client::RococoClient;

impl ClientCommon for RococoClient {
    const CHAIN: &'static str = "rococo";

    type Chain = bp_rococo::Rococo;
}
