//! Types used to connect to the Millau-Substrate chain.

use codec::Encode;
use relay_substrate_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// Millau header id.
pub type HeaderId = relay_utils::HeaderId<millau_runtime::Hash, millau_runtime::BlockNumber>;

/// Millau chain definition.
#[derive(Debug, Clone, Copy)]
pub struct Millau;

impl ChainBase for Millau {
	type BlockNumber = millau_runtime::BlockNumber;
	type Hash = millau_runtime::Hash;
	type Hasher = millau_runtime::Hashing;
	type Header = millau_runtime::Header;
}

impl Chain for Millau {
	const NAME: &'static str = "Millau";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(5);

	type AccountId = millau_runtime::AccountId;
	type Index = millau_runtime::Index;
	type SignedBlock = millau_runtime::SignedBlock;
	type Call = millau_runtime::Call;
}

impl ChainWithBalances for Millau {
	type NativeBalance = millau_runtime::Balance;

	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		use frame_support::storage::generator::StorageMap;
		StorageKey(
			frame_system::Account::<millau_runtime::Runtime>::storage_map_final_key(account_id),
		)
	}
}

impl TransactionSignScheme for Millau {
	type Chain = Millau;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = millau_runtime::UncheckedExtrinsic;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		signer_nonce: <Self::Chain as Chain>::Index,
		call: <Self::Chain as Chain>::Call,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::from_raw(
			call,
			(
				frame_system::CheckSpecVersion::<millau_runtime::Runtime>::new(),
				frame_system::CheckTxVersion::<millau_runtime::Runtime>::new(),
				frame_system::CheckGenesis::<millau_runtime::Runtime>::new(),
				frame_system::CheckEra::<millau_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
				frame_system::CheckNonce::<millau_runtime::Runtime>::from(signer_nonce),
				frame_system::CheckWeight::<millau_runtime::Runtime>::new(),
				pallet_transaction_payment::ChargeTransactionPayment::<millau_runtime::Runtime>::from(0),
			),
			(
				millau_runtime::VERSION.spec_version,
				millau_runtime::VERSION.transaction_version,
				genesis_hash,
				genesis_hash,
				(),
				(),
				(),
			),
		);
		let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
		let signer: sp_runtime::MultiSigner = signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		millau_runtime::UncheckedExtrinsic::new_signed(
			call,
			signer.into_account(),
			signature.into(),
			extra,
		)
	}
}

/// Millau signing params.
pub type SigningParams = sp_core::sr25519::Pair;

/// Millau header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<millau_runtime::Header>;
