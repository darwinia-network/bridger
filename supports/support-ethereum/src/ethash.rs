#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::error::BridgeEthereumError;
use codec::{Decode, Encode};
use sp_core::{H128, H512};

/// Ethash proof
#[derive(Clone, Encode, Decode, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct EthashProof {
    /// Dag nodes
    pub dag_nodes: [H512; 2],
    /// Merkle Proofs
    pub proof: Vec<H128>,
}

impl EthashProof {
    /// Generate EthashProof from hex array
    pub fn from_tuple(
        dag_nodes: [&str; 2],
        proof: [&str; 23],
    ) -> Result<EthashProof, BridgeEthereumError> {
        Ok(EthashProof {
            dag_nodes: [
                H512(array_bytes::hex2array(dag_nodes[0])?), // , 64
                H512(array_bytes::hex2array(dag_nodes[1])?), // , 64
            ],
            proof: proof
                .iter()
                .map(|s| H128(array_bytes::hex2array(*s))) // 16
                .collect::<Vec<H128>>(),
        })
    }
}

/// Json string format of `EthashProof`
#[derive(Serialize, Encode, Deserialize, PartialEq, Eq, Clone)]
pub struct EthashProofJson {
    dag_nodes: Vec<String>,
    proof: Vec<String>,
}

impl TryFrom<EthashProof> for EthashProofJson {
    type Error = BridgeEthereumError;

    fn try_from(that: EthashProof) -> Result<Self, Self::Error> {
        Ok(EthashProofJson {
            dag_nodes: that
                .dag_nodes
                .as_ref()
                .iter()
                .map(|n| array_bytes::bytes2hex("0x", n.0))
                .collect(),
            proof: that
                .proof
                .iter()
                .map(|p| array_bytes::bytes2hex("0x", p.0))
                .collect(),
        })
    }
}

impl TryFrom<EthashProofJson> for EthashProof {
    type Error = BridgeEthereumError;

    fn try_from(that: EthashProofJson) -> Result<Self, Self::Error> {
        let mut proof = Vec::with_capacity(that.proof.len());
        for item in that.proof {
            let bytes = array_bytes::hex2array(item)?; // 16
            proof.push(bytes);
        }
        Ok(Self {
            dag_nodes: [
                H512(array_bytes::hex2array(that.dag_nodes[0].as_str())?), // 64
                H512(array_bytes::hex2array(that.dag_nodes[1].as_str())?), // 64
            ],
            proof,
        })
    }
}
