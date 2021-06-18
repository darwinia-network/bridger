use bridge_component::ethereum_rpc::{EthereumRpcComponent, EthereumRpcConfig};
use bridge_component::http_client::{HttpClientComponent, HttpClientConfig};
use bridge_component::shadow::{ShadowComponent, ShadowConfig};

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

pub fn component_http_client(config: HttpClientConfig) -> HttpClientComponent {
    HttpClientComponent::new(config).expect("Failed to create http client component")
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
        .expect("Failed to create ethereum rpc component")
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
        .expect("Failed to create shadow component")
}

pub fn component_shadow_default<S: AsRef<str>>(ethereum_rpc_key: S) -> ShadowComponent {
    self::component_shadow(
        self::config_shadow(),
        self::config_ethereum_rpc(ethereum_rpc_key),
        self::config_http_client(),
    )
}
