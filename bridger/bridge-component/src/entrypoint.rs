use bridge_standard::bridge::chain::SubstrateChain;
use bridge_standard::bridge::config::Config;
use bridge_standard::bridge::task::BridgeSand;

use crate::component::bee::BeeComponent;
use crate::component::ethereum_rpc::EthereumRpcComponent;
use crate::component::http_client::HttpClientComponent;
use crate::component::microkv::MicrokvComponent;
use crate::component::shadow::ShadowComponent;
use crate::component::web3::Web3Component;
use crate::config::{
    BeeConfig, EthereumRpcConfig, HttpClientConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
use crate::error::ComponentResult;

pub struct Component {}

impl Component {
    pub fn namespace<S: AsRef<str>>(namespace: S) -> ComponentWithNamespace {
        ComponentWithNamespace {
            namespace: namespace.as_ref().to_string(),
        }
    }

    pub fn bee<T: BridgeSand, C: SubstrateChain>() -> ComponentResult<BeeComponent<C::ChainTypes>> {
        Self::namespace(Config::default_namespace()).bee::<T, C>()
    }

    pub fn http_client<T: BridgeSand>() -> ComponentResult<HttpClientComponent> {
        Self::namespace(Config::default_namespace()).http_client::<T>()
    }

    pub fn ethereum_rpc<T: BridgeSand>() -> ComponentResult<EthereumRpcComponent> {
        Self::namespace(Config::default_namespace()).ethereum_rpc::<T>()
    }

    pub fn shadow<T: BridgeSand>() -> ComponentResult<ShadowComponent> {
        Self::namespace(Config::default_namespace()).shadow::<T>()
    }

    pub fn web3<T: BridgeSand>() -> ComponentResult<Web3Component> {
        Self::namespace(Config::default_namespace()).web3::<T>()
    }

    pub fn microkv<T: BridgeSand>() -> ComponentResult<MicrokvComponent> {
        Self::namespace(Config::default_namespace()).microkv::<T>()
    }
}

pub struct ComponentWithNamespace {
    namespace: String,
}

impl ComponentWithNamespace {
    pub fn bee<T: BridgeSand, C: SubstrateChain>(
        &self,
    ) -> ComponentResult<BeeComponent<C::ChainTypes>> {
        let config: BeeConfig = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(BeeComponent::new(config))
    }

    pub fn http_client<T: BridgeSand>(&self) -> ComponentResult<HttpClientComponent> {
        let config: HttpClientConfig = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(HttpClientComponent::new(config))
    }

    pub fn ethereum_rpc<T: BridgeSand>(&self) -> ComponentResult<EthereumRpcComponent> {
        let config: EthereumRpcConfig = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(EthereumRpcComponent::new(config, self.http_client::<T>()?))
    }

    pub fn shadow<T: BridgeSand>(&self) -> ComponentResult<ShadowComponent> {
        let config: ShadowConfig = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(ShadowComponent::new(
            config,
            self.http_client::<T>()?,
            self.ethereum_rpc::<T>()?,
        ))
    }

    pub fn web3<T: BridgeSand>(&self) -> ComponentResult<Web3Component> {
        let config: Web3Config = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(Web3Component::new(config))
    }

    pub fn microkv<T: BridgeSand>(&self) -> ComponentResult<MicrokvComponent> {
        let config: MicrokvConfig = Config::restore_with_namespace(T::NAME, &self.namespace)?;
        Ok(MicrokvComponent::new(config))
    }
}
