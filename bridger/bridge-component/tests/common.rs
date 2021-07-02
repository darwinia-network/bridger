use bee_client::types::client::BasicSessionKeys;
use bee_client::types::client::ChainTypes;
use bee_client::types::substrate::balances::{AccountData, Balances};
use bee_client::types::substrate::events::EventTypeRegistry;
use bee_client::types::substrate::extra::DefaultExtra;
use bee_client::types::substrate::session::Session;
use bee_client::types::substrate::sudo::Sudo;
use bee_client::types::substrate::system::System;
use sp_runtime::generic::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
use sp_runtime::{MultiAddress, MultiSignature, OpaqueExtrinsic};

use bridge_component::component::bee::BeeComponent;
use bridge_component::component::ethereum_rpc::EthereumRpcComponent;
use bridge_component::component::http_client::HttpClientComponent;
use bridge_component::component::shadow::ShadowComponent;
use bridge_component::component::web3::Web3Component;
use bridge_component::config::{
    BeeConfig, EthereumRpcConfig, HttpClientConfig, ShadowConfig, Web3Config,
};

pub fn config_http_client() -> HttpClientConfig {
    HttpClientConfig { timeout: 30 }
}

pub fn config_ethereum_rpc<S: AsRef<str>>(api_key: S) -> EthereumRpcConfig {
    EthereumRpcConfig {
        rpc: vec![format!("https://mainnet.infura.io/v3/{}", api_key.as_ref())],
        atom: 0,
    }
}

pub fn config_shadow() -> ShadowConfig {
    ShadowConfig {
        endpoint: "https://shadow.darwinia.network".to_string(),
    }
}

pub fn config_web3<S: AsRef<str>>(api_key: S) -> Web3Config {
    Web3Config {
        endpoint: format!("https://mainnet.infura.io/v3/{}", api_key.as_ref()),
    }
}

pub fn config_bee() -> BeeConfig {
    BeeConfig {
        endpoint: "ws://101.32.220.161:9944".to_string(),
        strict: false,
    }
}

pub fn component_http_client(config: HttpClientConfig) -> HttpClientComponent {
    HttpClientComponent::new(config)
}

pub fn component_http_client_default() -> HttpClientComponent {
    self::component_http_client(self::config_http_client())
}

pub fn component_ethereum_rpc(
    config_ethereum_rpc: EthereumRpcConfig,
    config_http_client: HttpClientConfig,
) -> EthereumRpcComponent {
    let component_http_client = self::component_http_client(config_http_client);
    EthereumRpcComponent::new(config_ethereum_rpc, component_http_client)
}

pub fn component_ethereum_rpc_default<S: AsRef<str>>(api_key: S) -> EthereumRpcComponent {
    self::component_ethereum_rpc(
        self::config_ethereum_rpc(api_key),
        self::config_http_client(),
    )
}

pub fn component_shadow(
    config_shadow: ShadowConfig,
    config_ethereum_rpc: EthereumRpcConfig,
    config_http_client: HttpClientConfig,
) -> ShadowComponent {
    let component_http_client = self::component_http_client(config_http_client.clone());
    let component_ethereum_rpc =
        self::component_ethereum_rpc(config_ethereum_rpc, config_http_client);
    ShadowComponent::new(config_shadow, component_http_client, component_ethereum_rpc)
}

pub fn component_shadow_default<S: AsRef<str>>(ethereum_rpc_key: S) -> ShadowComponent {
    self::component_shadow(
        self::config_shadow(),
        self::config_ethereum_rpc(ethereum_rpc_key),
        self::config_http_client(),
    )
}

pub fn component_web3(config_web3: Web3Config) -> Web3Component {
    Web3Component::new(config_web3)
}

pub fn component_web3_default<S: AsRef<str>>(ethereum_rpc_key: S) -> Web3Component {
    self::component_web3(self::config_web3(ethereum_rpc_key))
}

pub fn component_bee<T: ChainTypes>() -> BeeComponent<T> {
    BeeComponent::<T>::new(self::config_bee())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestChainTypes;

impl ChainTypes for TestChainTypes {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system::<Self>();
        event_type_registry.with_balances::<Self>();
        event_type_registry.with_session::<Self>();
        // event_type_registry.with_staking();
        // event_type_registry.with_contracts();
        event_type_registry.with_sudo::<Self>();
        bee_client::types::client::register_default_type_sizes(event_type_registry);
    }
}

impl Balances for TestChainTypes {
    type Balance = u128;
}

impl System for TestChainTypes {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Session for TestChainTypes {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl Sudo for TestChainTypes {}
