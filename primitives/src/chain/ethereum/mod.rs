//! Ethereum types
// mod confirmation;
mod block;
mod ethash;
mod mmr;
mod parcel;
mod proof;
mod receipt;

pub use self::{
	block::{EthereumBlockRPC, EthereumHeader, EthereumHeaderJson},
	ethash::{EthashProof, EthashProofJson},
	mmr::{MMRProof, MMRProofJson, MMRRoot, MMRRootJson},
	parcel::{EthereumRelayHeaderParcel, EthereumRelayHeaderParcelJson},
	proof::{EthereumRelayProofs, EthereumRelayProofsJson},
	receipt::{
		EthReceiptBody, EthereumReceipt, EthereumReceiptProof, EthereumReceiptProofJson,
		EthereumReceiptProofThing, EthereumReceiptProofThingJson, RedeemFor,
	},
};
