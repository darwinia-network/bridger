use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;

use crate::bridge::{BridgeConfig, RelayConfig};
use crate::types::HexLaneId;

pub(crate) struct MessageRelay {
    pub relay_config: RelayConfig,
    pub client_pangolin: PangolinClient,
    pub client_pangoro: PangoroClient,
    pub subquery_pangoro: Subquery,
    pub subquery_pangolin: Subquery,
}

impl MessageRelay {
    pub async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let index_config = bridge_config.index;
        let config_pangolin = bridge_config.pangolin;
        let config_pangoro = bridge_config.pangoro;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_pangoro =
            PangoroClientComponent::component(config_pangoro.to_pangoro_client_config()?).await?;
        let subquery_pangoro =
            SubqueryComponent::component(index_config.pangoro, BridgeName::PangolinPangoro);
        let subquery_pangolin =
            SubqueryComponent::component(index_config.pangolin, BridgeName::PangolinPangoro);
        Ok(Self {
            relay_config: bridge_config.relay,
            client_pangolin,
            client_pangoro,
            subquery_pangoro,
            subquery_pangolin,
        })
    }
}

impl MessageRelay {
    pub fn lane(&self) -> Result<HexLaneId, BridgerError> {
        self.relay_config
            .lanes
            .clone()
            .get(0)
            .cloned()
            .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))
    }
}
