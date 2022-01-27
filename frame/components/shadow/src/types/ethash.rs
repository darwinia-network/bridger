use serde::{Deserialize, Serialize};

/// Json string format of `EthashProof`
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EthashProofJson {
    dag_nodes: Vec<String>,
    proof: Vec<String>,
}
