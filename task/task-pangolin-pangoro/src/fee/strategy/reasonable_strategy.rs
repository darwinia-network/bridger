use bridge_traits::bridge::component::BridgeComponent;
use component_subscan::{Subscan, SubscanComponent};

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;
use crate::task::PangolinPangoroTask;

#[derive(Clone)]
pub struct ReasonableStrategy {
    helper: StrategyHelper,
    subscan_pangolin: Subscan,
    subscan_pangoro: Subscan,
}

impl ReasonableStrategy {
    pub async fn new(&self) -> anyhow::Result<Self> {
        let helper = StrategyHelper::new().await?;
        let component_subcan_pangolin = SubscanComponent::restore_with_namespace::<
            PangolinPangoroTask,
        >("pangolin".to_string())?;
        let component_subscan_pangoro =
            SubscanComponent::restore_with_namespace::<PangolinPangoroTask>("pangoro".to_string())?;
        Ok(Self {
            helper,
            subscan_pangolin: component_subcan_pangolin.component().await?,
            subscan_pangoro: component_subscan_pangoro.component().await?,
        })
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for ReasonableStrategy {
    async fn handle(&self) -> anyhow::Result<()> {
        let top100_pangolin = self.subscan_pangolin.extrinsics(1, 100).await?;
        let top100_pangoro = self.subscan_pangoro.extrinsics(1, 100).await?;

        let code = top100_pangolin.code;
        Ok(())
    }
}
