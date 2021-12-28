use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GasPrice {
    pub(crate) code: i32,
    pub(crate) data: GasPriceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GasPriceData {
    pub(crate) rapid: u64,
    pub(crate) fast: u64,
    pub(crate) slow: u64,
    pub(crate) standard: u64,
    pub(crate) timestamp: u64,
}
