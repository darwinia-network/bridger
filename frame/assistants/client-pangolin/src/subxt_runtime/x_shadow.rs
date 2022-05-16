use shadow_liketh::types::{
    EthereumHeaderJson, EthereumReceiptWithMMRProof, HeaderParcel, MMRProofJson, ReceiptProof,
};

use crate::subxt_runtime::api::runtime_types;
use crate::subxt_runtime::api::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::subxt_runtime::api::runtime_types::{darwinia_bridge_ethereum, ethereum_primitives};
use crate::subxt_runtime::{ConvertTypeError, EthereumReceiptProofThing};

impl TryFrom<HeaderParcel> for EthereumRelayHeaderParcel {
    type Error = ConvertTypeError;

    fn try_from(value: HeaderParcel) -> Result<Self, Self::Error> {
        Ok(Self {
            header: value.header.try_into()?,
            parent_mmr_root: subxt::sp_core::H256(value.mmr_root),
        })
    }
}

impl TryFrom<web3::types::Block<web3::types::H256>> for ethereum_primitives::header::Header {
    type Error = ConvertTypeError;

    fn try_from(value: web3::types::Block<web3::types::H256>) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_hash: subxt::sp_core::H256(value.parent_hash.to_fixed_bytes()),
            timestamp: value.timestamp.as_u64(),
            number: value
                .number
                .ok_or_else(|| Self::Error::Other(format!("Missing ethereum block number")))?
                .as_u64(),
            author: runtime_types::primitive_types::H160(value.author.to_fixed_bytes()),
            transactions_root: subxt::sp_core::H256(value.transactions_root.to_fixed_bytes()),
            uncles_hash: subxt::sp_core::H256(value.uncles_hash.to_fixed_bytes()),
            extra_data: value.extra_data.0,
            state_root: subxt::sp_core::H256(value.state_root.to_fixed_bytes()),
            receipts_root: subxt::sp_core::H256(value.receipts_root.to_fixed_bytes()),
            log_bloom: runtime_types::ethbloom::Bloom(
                value
                    .logs_bloom
                    .ok_or_else(|| Self::Error::Other("The `logs_bloom` is required".to_string()))?
                    .to_fixed_bytes(),
            ),
            gas_used: runtime_types::primitive_types::U256(value.gas_used.0),
            gas_limit: runtime_types::primitive_types::U256(value.gas_limit.0),
            difficulty: runtime_types::primitive_types::U256(value.difficulty.0),
            seal: value
                .seal_fields
                .iter()
                .map(|v| v.0.clone())
                .collect::<Vec<Vec<u8>>>(),
            base_fee_per_gas: None,
            hash: value
                .hash
                .map(|item| subxt::sp_core::H256(item.to_fixed_bytes())),
        })
    }
}

impl TryFrom<EthereumHeaderJson> for ethereum_primitives::header::Header {
    type Error = ConvertTypeError;

    fn try_from(that: EthereumHeaderJson) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_hash: subxt::sp_core::H256(array_bytes::hex2array(&that.parent_hash)?),
            timestamp: that.timestamp,
            number: that.number,
            author: runtime_types::primitive_types::H160(array_bytes::hex2array(that.author)?), // bytes!(that.author.as_str(), 20),
            transactions_root: subxt::sp_core::H256(array_bytes::hex2array(
                that.transactions_root,
            )?),
            uncles_hash: subxt::sp_core::H256(array_bytes::hex2array(that.uncles_hash)?),
            extra_data: array_bytes::hex2bytes(that.extra_data)?, // no length
            state_root: subxt::sp_core::H256(array_bytes::hex2array(that.state_root)?),
            receipts_root: subxt::sp_core::H256(array_bytes::hex2array(that.receipts_root)?),
            log_bloom: runtime_types::ethbloom::Bloom(array_bytes::hex2array(that.log_bloom)?), // Bloom(bytes!(that.log_bloom.as_str(), 256)),
            gas_used: runtime_types::primitive_types::U256(
                subxt::sp_core::U256::from(that.gas_used).0,
            ),
            gas_limit: runtime_types::primitive_types::U256(
                subxt::sp_core::U256::from(that.gas_limit).0,
            ),
            difficulty: runtime_types::primitive_types::U256(
                subxt::sp_core::U256::from(that.difficulty).0,
            ),
            seal: {
                let mut rets = Vec::with_capacity(that.seal.len());
                for item in that.seal {
                    let bytes = array_bytes::hex2bytes(item)?;
                    rets.push(bytes);
                }
                rets
            },
            base_fee_per_gas: that
                .base_fee_per_gas
                .map(|v| runtime_types::primitive_types::U256(subxt::sp_core::U256::from(v).0)),
            hash: Some(subxt::sp_core::H256(array_bytes::hex2array(that.hash)?)), //Some(bytes!(that.hash.as_str(), 32)),
        })
    }
}

impl TryFrom<ReceiptProof> for ethereum_primitives::receipt::ReceiptProof {
    type Error = ConvertTypeError;

    fn try_from(that: ReceiptProof) -> Result<Self, Self::Error> {
        let index = if that.index.starts_with("0x") {
            &that.index[2..]
        } else {
            "00"
        };

        let hash = if !that.header_hash.is_empty() {
            array_bytes::hex2array(that.header_hash.as_str())?
        } else {
            [0; 32]
        };

        Ok(Self {
            index: u64::from_str_radix(index, 16).unwrap_or(0),
            proof: array_bytes::hex2bytes(that.proof)?,
            header_hash: subxt::sp_core::H256(hash),
        })
    }
}

impl TryFrom<MMRProofJson> for darwinia_bridge_ethereum::MMRProof {
    type Error = ConvertTypeError;

    fn try_from(that: MMRProofJson) -> Result<Self, Self::Error> {
        let proof = that
            .proof
            .iter()
            .map(|item| subxt::sp_core::H256(*item))
            .collect();
        Ok(Self {
            member_leaf_index: that.member_leaf_index,
            last_leaf_index: that.last_leaf_index,
            proof,
        })
    }
}

impl TryFrom<EthereumReceiptWithMMRProof> for EthereumReceiptProofThing {
    type Error = ConvertTypeError;

    fn try_from(value: EthereumReceiptWithMMRProof) -> Result<Self, Self::Error> {
        let receipt = value.receipt;
        Ok(Self {
            header: receipt.header.try_into()?,
            receipt_proof: receipt.receipt_proof.try_into()?,
            mmr_proof: value.mmr_proof.try_into()?,
        })
    }
}
