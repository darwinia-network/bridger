use client_pangolin::client::PangolinClient;
use postage::broadcast;

use client_pangolin::darwinia::client::Darwinia;
use client_pangolin::to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum};
use component_ethereum::ethereum::client::EthereumClient;
use component_subquery::Subquery;

use crate::bridge::ToExtrinsicsMessage;

pub struct ScanDataWrapper {
    /// Page from
    pub from: u64,
    /// Page limit
    pub limit: u32,
    /// Sender message to extrinsics
    pub sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    /// Subquery client
    pub subquery: Subquery,
    /// Ethereum client
    pub ethereum: EthereumClient,
    /// Pangolin client
    pub pangolin: PangolinClient,
}

impl ScanDataWrapper {
    pub fn sender_to_extrinsics_mut(&mut self) -> &mut broadcast::Sender<ToExtrinsicsMessage> {
        &mut self.sender_to_extrinsics
    }
}
