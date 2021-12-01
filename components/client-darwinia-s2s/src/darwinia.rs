use std::time::Duration;

use bridge_traits::bridge::chain::{BridgeChain, ChainCategory};
use codec::{Compact, Decode, Encode};
use relay_substrate_client::{
    BalanceOf, Chain, ChainBase, ChainWithBalances, IndexOf, TransactionEraOf,
    TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};

/// Pangolin header id.
pub type HeaderId = relay_utils::HeaderId<
    darwinia_common_primitives::Hash,
    darwinia_common_primitives::BlockNumber,
>;

/// Rialto header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<darwinia_common_primitives::Header>;

/// Millau chain definition.
#[derive(Debug, Clone, Copy)]
pub struct DarwiniaChain;

impl BridgeChain for DarwiniaChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Substrate;
}

impl ChainBase for DarwiniaChain {
    type BlockNumber = darwinia_common_primitives::BlockNumber;
    type Hash = darwinia_common_primitives::Hash;
    type Hasher = darwinia_common_primitives::Hashing;
    type Header = darwinia_common_primitives::Header;

    type AccountId = darwinia_common_primitives::AccountId;
    type Balance = darwinia_common_primitives::Balance;
    type Index = darwinia_common_primitives::Nonce;
    type Signature = darwinia_common_primitives::Signature;
}

impl Chain for DarwiniaChain {
    const NAME: &'static str = "Darwinia";
    const AVERAGE_BLOCK_INTERVAL: Duration =
        Duration::from_millis(darwinia_common_primitives::MILLISECS_PER_BLOCK);
    const STORAGE_PROOF_OVERHEAD: u32 = darwinia_bridge_primitives::EXTRA_STORAGE_PROOF_SIZE;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 =
        darwinia_bridge_primitives::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

    type SignedBlock = darwinia_runtime::SignedBlock;
    type Call = darwinia_runtime::Call;
    type WeightToFee = darwinia_runtime::WeightToFee;
}

impl ChainWithBalances for DarwiniaChain {
    fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
        use frame_support::storage::generator::StorageMap;
        StorageKey(
            frame_system::Account::<darwinia_runtime::Runtime>::storage_map_final_key(account_id),
        )
    }
}

impl TransactionSignScheme for DarwiniaChain {
    type Chain = DarwiniaChain;
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = darwinia_runtime::UncheckedExtrinsic;

    fn sign_transaction(
        genesis_hash: <Self::Chain as ChainBase>::Hash,
        signer: &Self::AccountKeyPair,
        _era: TransactionEraOf<Self::Chain>,
        unsigned: UnsignedTransaction<Self::Chain>,
    ) -> Self::SignedTransaction {
        let raw_payload = SignedPayload::from_raw(
            unsigned.call,
            (
                frame_system::CheckSpecVersion::<darwinia_runtime::Runtime>::new(),
                frame_system::CheckTxVersion::<darwinia_runtime::Runtime>::new(),
                frame_system::CheckGenesis::<darwinia_runtime::Runtime>::new(),
                frame_system::CheckEra::<darwinia_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
                frame_system::CheckNonce::<darwinia_runtime::Runtime>::from(unsigned.nonce),
                frame_system::CheckWeight::<darwinia_runtime::Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<darwinia_runtime::Runtime>::from(unsigned.tip),
                darwinia_bridge_ethereum::CheckEthereumRelayHeaderParcel::<darwinia_runtime::Runtime>::new(),
            ),
            (
                darwinia_runtime::VERSION.spec_version,
                darwinia_runtime::VERSION.transaction_version,
                genesis_hash,
                genesis_hash, // era.signed_payload(genesis_hash),
                (),
                (),
                (),
                (),
            ),
        );
        let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
        let signer: sp_runtime::MultiSigner = signer.public().into();
        let (call, extra, _) = raw_payload.deconstruct();

        darwinia_runtime::UncheckedExtrinsic::new_signed(
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
                let account_id: darwinia_common_primitives::AccountId =
                    (*signer.public().as_array_ref()).into();
                *address == darwinia_runtime::Address::from(account_id)
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
