use darwinia_common_primitives::AccountId;
use relay_utils::MaybeConnectionError;
use sp_core::Pair;

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;

/// Crazy strategy, always keep yourself as the first place in assigned relayers
pub struct CrazyStrategy {
    helper: StrategyHelper,
}

impl CrazyStrategy {
    pub async fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            helper: StrategyHelper::new().await?,
        })
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for CrazyStrategy {
    async fn handle(&mut self) -> color_eyre::Result<()> {
        let mut times = 0;
        loop {
            times += 1;
            if times > 3 {
                tracing::error!(
                    target: "darwinia-crab",
                    "[darwinia] Try reconnect many times({}), skip update fee (update fee strategy crazy)",
                    times
                );
                break;
            }
            match self.handle_darwinia().await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                target: "darwinia-crab",
                                "[darwinia] Try reconnect to chain (update fee strategy crazy)"
                            );
                            if let Err(re) = self.helper.reconnect_darwinia().await {
                                tracing::error!(
                                    target: "darwinia-crab",
                                    "[darwinia] Failed to reconnect substrate client: {:?} (update fee strategy crazy)",
                                    re
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        }

        times = 0;
        loop {
            times += 1;
            if times > 3 {
                tracing::error!(
                    target: "darwinia-crab",
                    "[crab] Try reconnect many times({}), skip update fee",
                    times
                );
                break;
            }
            match self.handle_crab().await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                target: "darwinia-crab",
                                "[crab] Try reconnect to chain (update fee strategy crazy)"
                            );
                            if let Err(re) = self.helper.reconnect_crab().await {
                                tracing::error!(
                                    target: "darwinia-crab",
                                    "[crab] Failed to reconnect substrate client: {:?} (update fee strategy crazy)",
                                    re
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl CrazyStrategy {
    async fn handle_darwinia(&mut self) -> color_eyre::Result<()> {
        let my_id = AccountId::from(self.helper.darwinia_signer().public().0);
        let darwinia_signer = self.helper.darwinia_signer().clone();

        let darwinia_api = self.helper.darwinia_api();

        if !darwinia_api.is_relayer(my_id.clone()).await? {
            tracing::warn!(
                target: "darwinia-crab",
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
        tracing::info!(
            target: "darwinia-crab",
            "[crazy] Update darwinia fee: {}",
            new_fee
        );
        darwinia_api
            .update_relay_fee(darwinia_signer, new_fee)
            .await?;
        Ok(())
    }

    async fn handle_crab(&mut self) -> color_eyre::Result<()> {
        let my_id = AccountId::from(self.helper.crab_signer().public().0);
        let crab_signer = self.helper.crab_signer().clone();
        let crab_api = self.helper.crab_api();

        if !crab_api.is_relayer(my_id.clone()).await? {
            tracing::warn!(
                target: "darwinia-crab",
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
        tracing::info!(
            target: "darwinia-crab",
            "[crazy] Update crab fee: {}",
            new_fee
        );
        crab_api.update_relay_fee(crab_signer, new_fee).await?;
        Ok(())
    }
}
