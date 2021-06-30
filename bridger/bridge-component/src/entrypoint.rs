use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, HttpClientConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
use bridge_config::Config;
use bridge_standard::bridge::chain::SubstrateChain;
use bridge_standard::bridge::task::BridgeSand;

use crate::component::bee::BeeComponent;
use crate::component::ethereum_rpc::EthereumRpcComponent;
use crate::component::http_client::HttpClientComponent;
use crate::component::microkv::MicrokvComponent;
use crate::component::shadow::ShadowComponent;
use crate::component::web3::Web3Component;
use crate::error::ComponentResult;

pub struct Component {}

impl Component {
    pub fn bee<T: BridgeSand, C: SubstrateChain>() -> ComponentResult<BeeComponent<C::ChainTypes>> {
        let config: BeeConfig = Config::restore(T::NAME)?;
        Ok(BeeComponent::new(config))
    }

    pub fn http_client<T: BridgeSand>() -> ComponentResult<HttpClientComponent> {
        let config: HttpClientConfig = Config::restore(T::NAME)?;
        Ok(HttpClientComponent::new(config))
    }

    pub fn ethereum_rpc<T: BridgeSand>() -> ComponentResult<EthereumRpcComponent> {
        let config: EthereumRpcConfig = Config::restore(T::NAME)?;
        Ok(EthereumRpcComponent::new(config, Self::http_client::<T>()?))
    }

    pub fn shadow<T: BridgeSand>() -> ComponentResult<ShadowComponent> {
        let config: ShadowConfig = Config::restore(T::NAME)?;
        Ok(ShadowComponent::new(
            config,
            Self::http_client::<T>()?,
            Self::ethereum_rpc::<T>()?,
        ))
    }

    pub fn web3<T: BridgeSand>() -> ComponentResult<Web3Component> {
        let config: Web3Config = Config::restore(T::NAME)?;
        Ok(Web3Component::new(config))
    }

    pub fn microkv<T: BridgeSand>() -> ComponentResult<MicrokvComponent> {
        let config: MicrokvConfig = Config::restore(T::NAME)?;
        Ok(MicrokvComponent::new(config))
    }
}
