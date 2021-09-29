use substrate_subxt::sp_core::{H160, H256};
use web3::types::Log;

use support_tracker::Tracker;
use support_tracker_evm_log::{EvmClient, EvmLogRangeData, LogsHandler};

use crate::toolkit::scanner::RopstenScanner;

/// Block Scanner
#[derive(Clone, Debug)]
pub struct RedeemScanner {
    scanner: RopstenScanner<RedeemHandler>,
}

impl RedeemScanner {
    pub fn new(tracker: Tracker) -> Self {
        let handler = RedeemHandler::new();
        let scanner = RopstenScanner::new(tracker, handler);
        Self { scanner }
    }
}

impl RedeemScanner {}

#[derive(Clone, Debug)]
struct RedeemHandler {}

impl RedeemHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl LogsHandler for RedeemHandler {
    async fn handle(&mut self, data: EvmLogRangeData) -> anyhow::Result<()> {
        let txs = data.transactions();
        todo!()
    }
}
