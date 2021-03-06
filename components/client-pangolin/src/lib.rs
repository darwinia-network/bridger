use std::time::Duration;

use bridge_traits::bridge::chain::{BridgeChain, ChainCategory};
use codec::Encode;
use relay_substrate_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};

/// Pangolin header id.
pub type HeaderId = relay_utils::HeaderId<drml_primitives::Hash, drml_primitives::BlockNumber>;

/// Rialto header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<drml_primitives::Header>;

/// Millau chain definition.
#[derive(Debug, Clone, Copy)]
pub struct PangolinChain;

impl BridgeChain for PangolinChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Substrate;
}

impl ChainBase for PangolinChain {
    type BlockNumber = drml_primitives::BlockNumber;
    type Hash = drml_primitives::Hash;
    type Hasher = drml_primitives::Hashing;
    type Header = drml_primitives::Header;
}

impl Chain for PangolinChain {
    const NAME: &'static str = "Pangolin";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

    type AccountId = drml_primitives::AccountId;
    type Index = drml_primitives::Nonce;
    type SignedBlock = pangolin_runtime::SignedBlock;
    type Call = pangolin_runtime::Call;
}

impl ChainWithBalances for PangolinChain {
    type NativeBalance = drml_primitives::Balance;

    fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
        use frame_support::storage::generator::StorageMap;
        StorageKey(
            frame_system::Account::<pangolin_runtime::Runtime>::storage_map_final_key(account_id),
        )
    }
}

impl TransactionSignScheme for PangolinChain {
    type Chain = PangolinChain;
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = pangolin_runtime::UncheckedExtrinsic;

    fn sign_transaction(
        genesis_hash: <Self::Chain as ChainBase>::Hash,
        signer: &Self::AccountKeyPair,
        signer_nonce: <Self::Chain as Chain>::Index,
        call: <Self::Chain as Chain>::Call,
    ) -> Self::SignedTransaction {
        let raw_payload = SignedPayload::from_raw(
            call,
            (
                frame_system::CheckSpecVersion::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckTxVersion::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckGenesis::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckEra::<pangolin_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
                frame_system::CheckNonce::<pangolin_runtime::Runtime>::from(signer_nonce),
                frame_system::CheckWeight::<pangolin_runtime::Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<pangolin_runtime::Runtime>::from(0),
                darwinia_ethereum_relay::CheckEthereumRelayHeaderParcel::<pangolin_runtime::Runtime>::new(),
            ),
            (
                pangolin_runtime::VERSION.spec_version,
                pangolin_runtime::VERSION.transaction_version,
                genesis_hash,
                genesis_hash,
                (),
                (),
                (),
                (),
            ),
        );
        let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
        let signer: sp_runtime::MultiSigner = signer.public().into();
        let (call, extra, _) = raw_payload.deconstruct();

        pangolin_runtime::UncheckedExtrinsic::new_signed(
            call,
            sp_runtime::MultiAddress::Id(signer.into_account()),
            signature.into(),
            extra,
        )
    }
}
