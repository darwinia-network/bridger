use bridge_config::Config;
use bridge_standard::bridge::chain::SubstrateChain;
use bridge_standard::bridge::task::BridgeTask;

use crate::component::bee::BeeComponent;
use crate::error::ComponentResult;
use bridge_config::config::component::BeeConfig;

pub struct Component {}

impl Component {
    pub fn bee<T: BridgeTask, C: SubstrateChain>() -> ComponentResult<BeeComponent<C::ChainTypes>> {
        let config: BeeConfig = Config::restore(T::NAME)?;
        Ok(BeeComponent::new(config))
    }
}
