use postage::broadcast;

use client_darwinia::client::DarwiniaClient;
use component_ethereum::ethereum::client::EthereumClient;
use subquery::Subquery;

use crate::bridge::ToExtrinsicsMessage;

pub struct ScanDataWrapper {
    pub from: u64,
    pub limit: u32,
    pub sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    pub subquery: Subquery,
    pub ethereum: EthereumClient,
    pub darwinia: DarwiniaClient,
}

impl ScanDataWrapper {
    pub fn sender_to_extrinsics_mut(&mut self) -> &mut broadcast::Sender<ToExtrinsicsMessage> {
        &mut self.sender_to_extrinsics
    }
}
