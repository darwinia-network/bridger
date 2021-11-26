use common_primitives::AccountId;
use sp_core::Pair;

use bridge_traits::bridge::task::BridgeSand;

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;
use crate::task::DarwiniaCrabTask;

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
        self.handle_darwinia().await?;
        self.handle_crab().await?;
        Ok(())
    }
}

impl CrazyStrategy {
    async fn handle_darwinia(&self) -> anyhow::Result<()> {
        let darwinia_api = self.helper.darwinia_api();
        let my_id = AccountId::from(self.helper.darwinia_signer().public().0);

        if !darwinia_api.is_relayer(my_id.clone()).await? {
            log::warn!(
                target: DarwiniaCrabTask::NAME,
                "You are not a relayer, please register first"
            );
            return Ok(());
        }

        // Query all assigned relayers
        let assigned_relayers = darwinia_api.assigned_relayers().await?;
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
        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[crazy] Update darwinia fee: {}",
            new_fee
        );
        darwinia_api
            .update_relay_fee(self.helper.darwinia_signer().clone(), new_fee)
            .await?;
        Ok(())
    }

    async fn handle_crab(&self) -> anyhow::Result<()> {
        let crab_api = self.helper.crab_api();
        let my_id = AccountId::from(self.helper.crab_signer().public().0);

        if !crab_api.is_relayer(my_id.clone()).await? {
            log::warn!(
                target: DarwiniaCrabTask::NAME,
                "You are not a relayer, please register first"
            );
            return Ok(());
        }

        // Query all assigned relayers
        let assigned_relayers = crab_api.assigned_relayers().await?;
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
        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[crazy] Update crab fee: {}",
            new_fee
        );
        crab_api
            .update_relay_fee(self.helper.crab_signer().clone(), new_fee)
            .await?;
        Ok(())
    }
}
