//! Types used to connect to the Pangoro-Substrate chain.

use bridge_traits::bridge::chain::{BridgeChain, ChainCategory};
use codec::{Compact, Decode, Encode};
use relay_substrate_client::{
    BalanceOf, Chain, ChainBase, ChainWithBalances, IndexOf, TransactionEraOf,
    TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// Pangoro header id.
pub type HeaderId = relay_utils::HeaderId<common_primitives::Hash, common_primitives::BlockNumber>;

/// Pangoro chain definition.
#[derive(Debug, Clone, Copy)]
pub struct PangoroChain;

impl BridgeChain for PangoroChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Substrate;
}

impl ChainBase for PangoroChain {
    type BlockNumber = common_primitives::BlockNumber;
    type Hash = common_primitives::Hash;
    type Hasher = common_primitives::Hashing;
    type Header = common_primitives::Header;

    type AccountId = common_primitives::AccountId;
    type Balance = common_primitives::Balance;
    type Index = common_primitives::Nonce;
    type Signature = common_primitives::Signature;
}

impl Chain for PangoroChain {
    const NAME: &'static str = "Pangoro";
    const AVERAGE_BLOCK_INTERVAL: Duration =
        Duration::from_millis(pangoro_constants::MILLISECS_PER_BLOCK);
    const STORAGE_PROOF_OVERHEAD: u32 = bridge_primitives::EXTRA_STORAGE_PROOF_SIZE;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = bridge_primitives::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

    type SignedBlock = pangoro_runtime::SignedBlock;
    type Call = pangoro_runtime::Call;
    type WeightToFee = pangoro_runtime::WeightToFee;
}

impl ChainWithBalances for PangoroChain {
    fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
        use frame_support::storage::generator::StorageMap;
        StorageKey(
            frame_system::Account::<pangoro_runtime::Runtime>::storage_map_final_key(account_id),
        )
    }
}

impl TransactionSignScheme for PangoroChain {
    type Chain = PangoroChain;
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = pangoro_runtime::UncheckedExtrinsic;

    fn sign_transaction(
        genesis_hash: <Self::Chain as ChainBase>::Hash,
        signer: &Self::AccountKeyPair,
        _era: TransactionEraOf<Self::Chain>,
        unsigned: UnsignedTransaction<Self::Chain>,
    ) -> Self::SignedTransaction {
        let raw_payload = SignedPayload::from_raw(
            unsigned.call,
            (
                frame_system::CheckSpecVersion::<pangoro_runtime::Runtime>::new(),
                frame_system::CheckTxVersion::<pangoro_runtime::Runtime>::new(),
                frame_system::CheckGenesis::<pangoro_runtime::Runtime>::new(),
                frame_system::CheckEra::<pangoro_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
                frame_system::CheckNonce::<pangoro_runtime::Runtime>::from(unsigned.nonce),
                frame_system::CheckWeight::<pangoro_runtime::Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<pangoro_runtime::Runtime>::from(unsigned.tip),
            ),
            (
                pangoro_runtime::VERSION.spec_version,
                pangoro_runtime::VERSION.transaction_version,
                genesis_hash,
                genesis_hash, //era.signed_payload(genesis_hash),
                (),
                (),
                (),
            ),
        );
        let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
        let signer: sp_runtime::MultiSigner = signer.public().into();
        let (call, extra, _) = raw_payload.deconstruct();

        pangoro_runtime::UncheckedExtrinsic::new_signed(
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
                let account_id: common_primitives::AccountId =
                    (*signer.public().as_array_ref()).into();
                *address == pangoro_runtime::Address::from(account_id)
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

/// Pangoro signing params.
pub type SigningParams = sp_core::sr25519::Pair;

/// Pangoro header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<common_primitives::Header>;
