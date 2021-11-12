use bridge_traits::bridge::task::BridgeSand;
use common_primitives::AccountId;
use sp_core::Pair;

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;
use crate::task::PangolinPangoroTask;

/// Crazy strategy, always keep yourself as the first place in assigned relayers
pub struct CrazyStrategy {
    helper: StrategyHelper,
}

impl CrazyStrategy {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            helper: StrategyHelper::new().await?,
        })
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for CrazyStrategy {
    async fn handle(&self) -> anyhow::Result<()> {
        self.handle_pangolin().await?;
        self.handle_pangoro().await?;
        Ok(())
    }
}

impl CrazyStrategy {
    async fn handle_pangolin(&self) -> anyhow::Result<()> {
        let pangolin_api = self.helper.pangolin_api();
        let my_id = AccountId::from(self.helper.pangolin_signer().public().0);

        if !pangolin_api.is_relayer(my_id.clone()).await? {
            log::warn!(
                target: PangolinPangoroTask::NAME,
                "You are not a relayer, please register first"
            );
            return Ok(());
        }

        // Query all assigned relayers
        let assigned_relayers = pangolin_api.assigned_relayers().await?;
        let min_fee = match assigned_relayers.get(0) {
            Some(relayer) => {
                if relayer.id == my_id {
                    // If you are the first assigned relayer, no change will be made
                    return Ok(());
                }
                relayer.fee
            }
            None => 51, // This is default value when not have any assigned relayers
        };

        // Nice (
        // RISK: If the cost is not judged, it may be a negative benefit.
        let new_fee = min_fee - 1;
        pangolin_api
            .update_relay_fee(self.helper.pangolin_signer().clone(), new_fee)
            .await?;
        Ok(())
    }

    async fn handle_pangoro(&self) -> anyhow::Result<()> {
        let pangoro_api = self.helper.pangoro_api();
        let my_id = AccountId::from(self.helper.pangoro_signer().public().0);

        if !pangoro_api.is_relayer(my_id.clone()).await? {
            log::warn!(
                target: PangolinPangoroTask::NAME,
                "You are not a relayer, please register first"
            );
            return Ok(());
        }

        // Query all assigned relayers
        let assigned_relayers = pangoro_api.assigned_relayers().await?;
        let min_fee = match assigned_relayers.get(0) {
            Some(relayer) => {
                if relayer.id == my_id {
                    // If you are the first assigned relayer, no change will be made
                    return Ok(());
                }
                relayer.fee
            }
            None => 51, // This is default value when not have any assigned relayers
        };

        // Nice (
        // RISK: If the cost is not judged, it may be a negative benefit.
        let new_fee = min_fee - 1;
        pangoro_api
            .update_relay_fee(self.helper.pangoro_signer().clone(), new_fee)
            .await?;
        Ok(())
    }
}
