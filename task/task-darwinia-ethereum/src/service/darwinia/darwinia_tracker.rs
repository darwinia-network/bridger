use std::time::Duration;

use substrate_subxt::sp_runtime::generic::Header;
use substrate_subxt::sp_runtime::traits::BlakeTwo256;
use tokio::time::sleep;

use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::darwinia::client::Darwinia;
use component_state::state::BridgeState;

use crate::error::Result;
use crate::task::DarwiniaEthereumTask;

/// DarwiniaTracker
pub struct DarwiniaBlockTracker {
    darwinia: Darwinia,
    state: BridgeState,
}

impl DarwiniaBlockTracker {
    /// new
    pub fn new(darwinia: Darwinia, state: BridgeState) -> Self {
        Self { darwinia, state }
    }

    /// get next block
    pub async fn next_block(&mut self) -> Result<Header<u32, BlakeTwo256>> {
        loop {
            match self.get_next_block().await {
                Ok(result) => {
                    if let Some(header) = result {
                        return Ok(header);
                    } else {
                        sleep(Duration::from_secs(6)).await;
                    }
                }
                Err(err) => {
                    error!(
                        target: DarwiniaEthereumTask::NAME,
                        "An error occurred while tracking next darwinia block: {:#?}", err
                    );
                    let err_msg = format!("{:?}", err).to_lowercase();
                    if err_msg.contains("restart") {
                        return Err(crate::error::Error::RestartFromJsonrpsee.into());
                    } else {
                        sleep(Duration::from_secs(30)).await;
                    }
                }
            }
        }
    }

    async fn get_next_block(&mut self) -> Result<Option<Header<u32, BlakeTwo256>>> {
        let kv = self.state.microkv();
        let next_block = kv.get("last-tracked-darwinia-block")?.unwrap_or(0u32) + 1;
        let finalized_block_hash = self.darwinia.finalized_head().await?;
        match self
            .darwinia
            .get_block_number_by_hash(finalized_block_hash)
            .await?
        {
            Some(finalized_block_number) => {
                if next_block > finalized_block_number {
                    Ok(None)
                } else {
                    let header = self.darwinia.get_block_by_number(next_block).await?;
                    // kv.put("last-tracked-darwinia-block", &next_block);
                    Ok(Some(header))
                }
            }
            None => Ok(None),
        }
    }
}
