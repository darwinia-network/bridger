use drml_common_primitives::AccountId;
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
                    target: "pangolin-pangoro",
                    "[pangolin] Try reconnect many times({}), skip update fee (update fee strategy crazy)",
                    times
                );
                break;
            }
            match self.handle_pangolin().await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                target: "pangolin-pangoro",
                                "[pangolin] Try reconnect to chain (update fee strategy crazy)"
                            );
                            if let Err(re) = self.helper.reconnect_pangolin().await {
                                tracing::error!(
                                    target: "pangolin-pangoro",
                                    "[pangolin] Failed to reconnect substrate client: {:?} (update fee strategy crazy)",
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
                    target: "pangolin-pangoro",
                    "[pangolin] Try reconnect many times({}), skip update fee",
                    times
                );
                break;
            }
            match self.handle_pangoro().await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                target: "pangolin-pangoro",
                                "[pangoro] Try reconnect to chain (update fee strategy crazy)"
                            );
                            if let Err(re) = self.helper.reconnect_pangoro().await {
                                tracing::error!(
                                    target: "pangolin-pangoro",
                                    "[pangoro] Failed to reconnect substrate client: {:?} (update fee strategy crazy)",
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
    async fn handle_pangolin(&mut self) -> color_eyre::Result<()> {
        let my_id = AccountId::from(self.helper.pangolin_signer().public().0);
        let pangolin_signer = self.helper.pangolin_signer().clone();
        let pangolin_api = self.helper.pangolin_api();

        if !pangolin_api.is_relayer(my_id.clone()).await? {
            tracing::warn!(
                target: "pangolin-pangoro",
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
        tracing::info!(
            target: "pangolin-pangoro",
            "[crazy] Update pangolin fee: {}",
            new_fee
        );
        pangolin_api
            .update_relay_fee(pangolin_signer, new_fee)
            .await?;
        Ok(())
    }

    async fn handle_pangoro(&mut self) -> color_eyre::Result<()> {
        let my_id = AccountId::from(self.helper.pangoro_signer().public().0);
        let pangoro_signer = self.helper.pangoro_signer().clone();
        let pangoro_api = self.helper.pangoro_api();

        if !pangoro_api.is_relayer(my_id.clone()).await? {
            tracing::warn!(
                target: "pangolin-pangoro",
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
        tracing::info!(
            target: "pangolin-pangoro",
            "[crazy] Update pangoro fee: {}",
            new_fee
        );
        pangoro_api
            .update_relay_fee(pangoro_signer, new_fee)
            .await?;
        Ok(())
    }
}
