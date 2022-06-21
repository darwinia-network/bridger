use abstract_client_s2s::client::S2SClientRelay;
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

pub(crate) struct MessageRelay<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub relay_config: RelayConfig,
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub subquery_target: Subquery,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> MessageRelay<SC, TC> {
    pub async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let index_config = bridge_config.index;
        let config_source = bridge_config.pangolin;
        let config_target = bridge_config.pangoro;

        let client_source = config_source.to_pangolin_client().await?;
        let client_target = config_target.to_pangoro_client().await?;
        let subquery_source = index_config.to_pangolin_subquery()?;
        let subquery_target = index_config.to_pangoro_subquery()?;
        Ok(Self {
            relay_config: bridge_config.relay,
            client_source,
            client_target,
            subquery_source,
            subquery_target,
        })
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> MessageRelay<SC, TC> {
    pub fn lane(&self) -> Result<HexLaneId, BridgerError> {
        self.relay_config
            .lanes
            .clone()
            .get(0)
            .cloned()
            .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))
    }
}
