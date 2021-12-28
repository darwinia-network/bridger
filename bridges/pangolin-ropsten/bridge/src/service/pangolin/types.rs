use postage::broadcast;

use client_pangolin::darwinia::client::Darwinia;
use client_pangolin::to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum};
use component_ethereum::ethereum::client::EthereumClient;
use component_subquery::Subquery;

use crate::bridge::ToExtrinsicsMessage;

pub struct ScanDataWrapper {
    pub from: u64,
    pub limit: u32,
    pub sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    pub subquery: Subquery,
    pub darwinia: Darwinia,
    pub ethereum: EthereumClient,
    pub darwinia2ethereum: Darwinia2Ethereum,
    pub account: ToEthereumAccount,
}

impl ScanDataWrapper {
    pub fn sender_to_extrinsics_mut(&mut self) -> &mut broadcast::Sender<ToExtrinsicsMessage> {
        &mut self.sender_to_extrinsics
    }
}
