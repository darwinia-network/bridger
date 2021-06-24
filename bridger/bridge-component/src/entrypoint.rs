use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, HttpClientConfig, ShadowConfig, Web3Config,
};
use bridge_config::Config;
use bridge_standard::bridge::chain::SubstrateChain;
use bridge_standard::bridge::task::BridgeTask;

use crate::component::bee::BeeComponent;
use crate::component::ethereum_rpc::EthereumRpcComponent;
use crate::component::http_client::HttpClientComponent;
use crate::component::shadow::ShadowComponent;
use crate::component::web3::Web3Component;
use crate::error::ComponentResult;

pub struct Component {}

impl Component {
    pub fn bee<T: BridgeTask, C: SubstrateChain>() -> ComponentResult<BeeComponent<C::ChainTypes>> {
        let config: BeeConfig = Config::restore(T::NAME)?;
        Ok(BeeComponent::new(config))
    }

    pub fn http_client<T: BridgeTask>() -> ComponentResult<HttpClientComponent> {
        let config: HttpClientConfig = Config::restore(T::NAME)?;
        Ok(HttpClientComponent::new(config))
    }

    pub fn ethereum_rpc<T: BridgeTask>() -> ComponentResult<EthereumRpcComponent> {
        let config: EthereumRpcConfig = Config::restore(T::NAME)?;
        Ok(EthereumRpcComponent::new(config, Self::http_client::<T>()?))
    }

    pub fn shadow<T: BridgeTask>() -> ComponentResult<ShadowComponent> {
        let config: ShadowConfig = Config::restore(T::NAME)?;
        Ok(ShadowComponent::new(
            config,
            Self::http_client::<T>()?,
            Self::ethereum_rpc::<T>()?,
        ))
    }

    pub fn web3<T: BridgeTask>() -> ComponentResult<Web3Component> {
        let config: Web3Config = Config::restore(T::NAME)?;
        Ok(Web3Component::new(config))
    }
}
