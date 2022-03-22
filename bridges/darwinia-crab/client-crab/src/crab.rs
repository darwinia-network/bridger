//! Types used to connect to the Crab-Substrate chain.

use std::time::Duration;

use bp_messages::MessageNonce;
use codec::{Compact, Decode, Encode};
use frame_support::dispatch::Weight;
use relay_substrate_client::{
    BalanceOf, Chain, ChainBase, ChainWithBalances, ChainWithMessages, IndexOf, SignParam,
    TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};

/// Crab header id.
pub type HeaderId = relay_utils::HeaderId<
    darwinia_common_primitives::Hash,
    darwinia_common_primitives::BlockNumber,
>;

/// Crab chain definition.
#[derive(Debug, Clone, Copy)]
pub struct CrabChain;

impl ChainBase for CrabChain {
    type BlockNumber = darwinia_common_primitives::BlockNumber;
    type Hash = darwinia_common_primitives::Hash;
    type Hasher = darwinia_common_primitives::Hashing;
    type Header = darwinia_common_primitives::Header;

    type AccountId = darwinia_common_primitives::AccountId;
    type Balance = darwinia_common_primitives::Balance;
    type Index = darwinia_common_primitives::Nonce;
    type Signature = darwinia_common_primitives::Signature;

    fn max_extrinsic_size() -> u32 {
        darwinia_bridge_primitives::Crab::max_extrinsic_size()
    }

    fn max_extrinsic_weight() -> Weight {
        darwinia_bridge_primitives::Crab::max_extrinsic_weight()
    }
}

impl Chain for CrabChain {
    const NAME: &'static str = "Crab";
    const TOKEN_ID: Option<&'static str> = None;
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
        darwinia_bridge_primitives::BEST_FINALIZED_CRAB_HEADER_METHOD;
    const AVERAGE_BLOCK_INTERVAL: Duration =
        Duration::from_millis(darwinia_common_primitives::MILLISECS_PER_BLOCK);
    const STORAGE_PROOF_OVERHEAD: u32 = darwinia_bridge_primitives::EXTRA_STORAGE_PROOF_SIZE;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 =
        darwinia_bridge_primitives::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

    type SignedBlock = crab_runtime::SignedBlock;
    type Call = crab_runtime::Call;
    type WeightToFee = crab_runtime::WeightToFee;
}

impl ChainWithMessages for CrabChain {
    const WITH_CHAIN_MESSAGES_PALLET_NAME: &'static str =
        darwinia_bridge_primitives::WITH_CRAB_MESSAGES_PALLET_NAME;
    const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
        darwinia_bridge_primitives::TO_CRAB_MESSAGE_DETAILS_METHOD;
    const TO_CHAIN_LATEST_GENERATED_NONCE_METHOD: &'static str =
        darwinia_bridge_primitives::TO_CRAB_LATEST_GENERATED_NONCE_METHOD;
    const TO_CHAIN_LATEST_RECEIVED_NONCE_METHOD: &'static str =
        darwinia_bridge_primitives::TO_CRAB_LATEST_RECEIVED_NONCE_METHOD;
    const FROM_CHAIN_LATEST_RECEIVED_NONCE_METHOD: &'static str =
        darwinia_bridge_primitives::FROM_CRAB_LATEST_RECEIVED_NONCE_METHOD;
    const FROM_CHAIN_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
        darwinia_bridge_primitives::FROM_CRAB_LATEST_CONFIRMED_NONCE_METHOD;
    const FROM_CHAIN_UNREWARDED_RELAYERS_STATE: &'static str =
        darwinia_bridge_primitives::FROM_CRAB_UNREWARDED_RELAYERS_STATE;
    const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_CHAIN: Weight =
        darwinia_bridge_primitives::PAY_INBOUND_DISPATCH_FEE_WEIGHT;
    const MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX: MessageNonce =
        darwinia_bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
    const MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX: MessageNonce =
        darwinia_bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
    type WeightInfo = ();
}

impl ChainWithBalances for CrabChain {
    fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
        use frame_support::storage::generator::StorageMap;
        StorageKey(
            frame_system::Account::<crab_runtime::Runtime>::storage_map_final_key(account_id),
        )
    }
}

impl TransactionSignScheme for CrabChain {
    type Chain = CrabChain;
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = crab_runtime::UncheckedExtrinsic;

    fn sign_transaction(param: SignParam<Self>) -> Self::SignedTransaction {
        let raw_payload = SignedPayload::from_raw(
            param.unsigned.call.clone(),
            (
                frame_system::CheckSpecVersion::<crab_runtime::Runtime>::new(),
                frame_system::CheckTxVersion::<crab_runtime::Runtime>::new(),
                frame_system::CheckGenesis::<crab_runtime::Runtime>::new(),
                frame_system::CheckEra::<crab_runtime::Runtime>::from(
                    sp_runtime::generic::Era::Immortal,
                ),
                frame_system::CheckNonce::<crab_runtime::Runtime>::from(param.unsigned.nonce),
                frame_system::CheckWeight::<crab_runtime::Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<crab_runtime::Runtime>::from(
                    param.unsigned.tip,
                ),
            ),
            (
                param.spec_version,
                param.transaction_version,
                param.genesis_hash,
                param.genesis_hash, //era.signed_payload(genesis_hash),
                (),
                (),
                (),
            ),
        );
        let signature = raw_payload.using_encoded(|payload| param.signer.sign(payload));
        let signer: sp_runtime::MultiSigner = param.signer.public().into();
        let (call, extra, _) = raw_payload.deconstruct();

        crab_runtime::UncheckedExtrinsic::new_signed(
            call,
            sp_runtime::MultiAddress::Id(signer.into_account()),
            signature.into(),
            extra,
        )
    }

    fn is_signed(tx: &Self::SignedTransaction) -> bool {
        tx.0.signature.is_some()
    }

    fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
        tx.0.signature
            .as_ref()
            .map(|(address, _, _)| {
                let account_id: darwinia_common_primitives::AccountId =
                    (*signer.public().as_array_ref()).into();
                *address == crab_runtime::Address::from(account_id)
            })
            .unwrap_or(false)
    }

    fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self::Chain>> {
        let extra = &tx.0.signature.as_ref()?.2;
        Some(UnsignedTransaction {
            call: tx.0.function,
            nonce: Compact::<IndexOf<Self::Chain>>::decode(&mut &extra.4.encode()[..])
                .ok()?
                .into(),
            tip: Compact::<BalanceOf<Self::Chain>>::decode(&mut &extra.6.encode()[..])
                .ok()?
                .into(),
        })
    }
}
