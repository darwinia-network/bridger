#![cfg(features = "runtime")]
use super::EthereumHeader;
use codec::{Codec, Decode};
use ethereum_primitives::header::EthereumHeader as DarwiniaEthereumHeader;

impl Into<EthereumHeader> for DarwiniaEthereumHeader {
    fn into(self) -> EthereumHeader {
        self.encode().decode()
    }
}
