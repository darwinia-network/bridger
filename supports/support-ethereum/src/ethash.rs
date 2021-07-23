#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};

use bridge_primitives::{
    array::{H128, H512},
    hex,
};

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
    pub fn from_tuple(dag_nodes: [&str; 2], proof: [&str; 23]) -> EthashProof {
        EthashProof {
            dag_nodes: [
                H512(bridge_primitives::bytes!(dag_nodes[0], 64)),
                H512(bridge_primitives::bytes!(dag_nodes[1], 64)),
            ],
            proof: proof
                .iter()
                .map(|s| H128(bridge_primitives::bytes!(*s, 16)))
                .collect::<Vec<H128>>(),
        }
    }
}

/// Json string format of `EthashProof`
#[derive(Serialize, Encode, Deserialize, PartialEq, Eq, Clone)]
pub struct EthashProofJson {
    dag_nodes: Vec<String>,
    proof: Vec<String>,
}

impl From<EthashProof> for EthashProofJson {
    fn from(that: EthashProof) -> Self {
        EthashProofJson {
            dag_nodes: that
                .dag_nodes
                .as_ref()
                .iter()
                .map(|n| format!("0x{}", hex!(n.0.to_vec())))
                .collect(),
            proof: that
                .proof
                .iter()
                .map(|p| format!("0x{}", hex!(p.0.to_vec())))
                .collect(),
        }
    }
}

impl From<EthashProofJson> for EthashProof {
    fn from(that: EthashProofJson) -> Self {
        EthashProof {
            dag_nodes: [
                H512(bridge_primitives::bytes!(that.dag_nodes[0].as_str(), 64)),
                H512(bridge_primitives::bytes!(that.dag_nodes[1].as_str(), 64)),
            ],
            proof: that
                .proof
                .iter()
                .map(|p| H128(bridge_primitives::bytes!(p.as_str(), 16)))
                .collect(),
        }
    }
}
