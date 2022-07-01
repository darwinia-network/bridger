use codec::Encode;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

use crate::subxt_runtime::api::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::subxt_runtime::api::runtime_types::{darwinia_bridge_ethereum, ethereum_primitives};

/// Ethereum receipt proof
#[derive(Clone, Debug, codec::Encode, codec::Decode)]
pub struct EthereumReceiptProofThing {
    /// Ethereum header
    pub header: ethereum_primitives::header::Header,
    /// Receipt proof
    pub receipt_proof: ethereum_primitives::receipt::ReceiptProof,
    /// MMR proof
    pub mmr_proof: darwinia_bridge_ethereum::MMRProof,
}

// --
// todo: there is not good way.

impl serde::Serialize for EthereumReceiptProofThing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = self.encode();
        let hex = array_bytes::bytes2hex("", encoded.as_slice());
        serializer.serialize_str(&hex)
    }
}

impl<'de> Deserialize<'de> for EthereumReceiptProofThing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = String::deserialize(deserializer)?;
        let bytes = array_bytes::hex2bytes(hex.as_ref())
            .map_err(|_e| D::Error::custom("Wrong serialize bytes, can't convert to bytes"))?;
        let ret = codec::Decode::decode(&mut bytes.as_slice())
            .map_err(|_e| D::Error::custom("Wrong serialize bytes, can't decode to struct."))?;
        Ok(ret)
    }
}

impl serde::Serialize for EthereumRelayHeaderParcel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = self.encode();
        let hex = array_bytes::bytes2hex("", encoded.as_slice());
        serializer.serialize_str(&hex)
    }
}

impl<'de> Deserialize<'de> for EthereumRelayHeaderParcel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = String::deserialize(deserializer)?;
        let bytes = array_bytes::hex2bytes(hex.as_ref())
            .map_err(|_e| D::Error::custom("Wrong serialize bytes, can't convert to bytes"))?;
        let ret = codec::Decode::decode(&mut bytes.as_slice())
            .map_err(|_e| D::Error::custom("Wrong serialize bytes, can't decode to struct."))?;
        Ok(ret)
    }
}
