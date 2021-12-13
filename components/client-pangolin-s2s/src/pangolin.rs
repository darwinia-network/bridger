use std::time::Duration;

use bridge_traits::bridge::chain::{BridgeChain, ChainCategory};
use codec::{Compact, Decode, Encode};
use relay_substrate_client::{
    BalanceOf, Chain, ChainBase, ChainWithBalances, IndexOf, SignParam, TransactionSignScheme,
    UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};

/// Pangolin header id.
pub type HeaderId =
    relay_utils::HeaderId<drml_common_primitives::Hash, drml_common_primitives::BlockNumber>;

/// Rialto header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<drml_common_primitives::Header>;

/// Millau chain definition.
#[derive(Debug, Clone, Copy)]
pub struct PangolinChain;

impl BridgeChain for PangolinChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Substrate;
}

impl ChainBase for PangolinChain {
    type BlockNumber = drml_common_primitives::BlockNumber;
    type Hash = drml_common_primitives::Hash;
    type Hasher = drml_common_primitives::Hashing;
    type Header = drml_common_primitives::Header;

    type AccountId = drml_common_primitives::AccountId;
    type Balance = drml_common_primitives::Balance;
    type Index = drml_common_primitives::Nonce;
    type Signature = drml_common_primitives::Signature;
}

impl Chain for PangolinChain {
    const NAME: &'static str = "Pangolin";
    const AVERAGE_BLOCK_INTERVAL: Duration =
        Duration::from_millis(drml_common_primitives::MILLISECS_PER_BLOCK);
    const STORAGE_PROOF_OVERHEAD: u32 = drml_bridge_primitives::EXTRA_STORAGE_PROOF_SIZE;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 =
        drml_bridge_primitives::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

    type SignedBlock = pangolin_runtime::SignedBlock;
    type Call = pangolin_runtime::Call;
    type WeightToFee = pangolin_runtime::WeightToFee;
}

impl ChainWithBalances for PangolinChain {
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

    fn sign_transaction(param: SignParam<Self>) -> Self::SignedTransaction {
        let raw_payload = SignedPayload::from_raw(
            param.unsigned.call.clone(),
            (
                frame_system::CheckSpecVersion::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckTxVersion::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckGenesis::<pangolin_runtime::Runtime>::new(),
                frame_system::CheckEra::<pangolin_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
                frame_system::CheckNonce::<pangolin_runtime::Runtime>::from(param.unsigned.nonce),
                frame_system::CheckWeight::<pangolin_runtime::Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<pangolin_runtime::Runtime>::from(param.unsigned.tip),
                darwinia_bridge_ethereum::CheckEthereumRelayHeaderParcel::<pangolin_runtime::Runtime>::new(),
            ),
            (
                param.spec_version,
                param.transaction_version,
                param.genesis_hash,
                param.genesis_hash, // era.signed_payload(genesis_hash),
                (),
                (),
                (),
                (),
            ),
        );
        let signature = raw_payload.using_encoded(|payload| param.signer.sign(payload));
        let signer: sp_runtime::MultiSigner = param.signer.public().into();
        let (call, extra, _) = raw_payload.deconstruct();

        pangolin_runtime::UncheckedExtrinsic::new_signed(
            call,
            sp_runtime::MultiAddress::Id(signer.into_account()),
            signature.into(),
            extra,
        )
    }

    fn is_signed(tx: &Self::SignedTransaction) -> bool {
        tx.signature.is_some()
    }

    fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
        tx.signature
            .as_ref()
            .map(|(address, _, _)| {
                let account_id: drml_common_primitives::AccountId =
                    (*signer.public().as_array_ref()).into();
                *address == pangolin_runtime::Address::from(account_id)
            })
            .unwrap_or(false)
    }

    fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self::Chain>> {
        let extra = &tx.signature.as_ref()?.2;
        Some(UnsignedTransaction {
            call: tx.function,
            nonce: Compact::<IndexOf<Self::Chain>>::decode(&mut &extra.4.encode()[..])
                .ok()?
                .into(),
            tip: Compact::<BalanceOf<Self::Chain>>::decode(&mut &extra.6.encode()[..])
                .ok()?
                .into(),
        })
    }
}
