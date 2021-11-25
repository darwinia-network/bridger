use bridge_traits::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default)]
pub struct SubqueryConfig {
    pub endpoint: string,
}

impl BridgeConfig for SubqueryConfig {
    fn marker() -> &'static str {
        "component-subquery"
    }

    fn template() -> Self {
        Self {
            endpoint: "https://api.subquery.network/sq/darwinia-network/pangolin-bridger"
                .to_string(),
        }
    }
}
