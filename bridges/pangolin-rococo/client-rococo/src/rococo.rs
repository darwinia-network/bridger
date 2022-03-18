//! Types used to connect to the Rococo-Substrate chain.

use frame_support::weights::Weight;
use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;


#[derive(Debug, Clone, Copy)]
pub struct RococoChain;

impl ChainBase for RococoChain {
    type BlockNumber = drml_common_primitives::BlockNumber;
    type Hash = drml_common_primitives::Hash;
    type Hasher = drml_common_primitives::Hashing;
    type Header = drml_common_primitives::Header;

    type AccountId = drml_common_primitives::AccountId;
    type Balance = drml_common_primitives::Balance;
    type Index = drml_common_primitives::Nonce;
    type Signature = drml_common_primitives::Signature;

    fn max_extrinsic_size() -> u32 {
        drml_bridge_primitives::Rococo::max_extrinsic_size()
    }

    fn max_extrinsic_weight() -> Weight {
        drml_bridge_primitives::Rococo::max_extrinsic_weight()
    }
}

impl Chain for RococoChain {
    const NAME: &'static str = "Rococo";
    const TOKEN_ID: Option<&'static str> = None;
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
        drml_bridge_primitives::BEST_FINALIZED_PANGORO_HEADER_METHOD;
    const AVERAGE_BLOCK_INTERVAL: Duration =
        Duration::from_millis(drml_common_primitives::MILLISECS_PER_BLOCK);
    const STORAGE_PROOF_OVERHEAD: u32 = drml_bridge_primitives::EXTRA_STORAGE_PROOF_SIZE;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 =
        drml_bridge_primitives::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

    type SignedBlock = rococo_runtime::SignedBlock;
    type Call = pangolin_runtime::Call;
    type WeightToFee = pangolin_runtime::WeightToFee;
}
