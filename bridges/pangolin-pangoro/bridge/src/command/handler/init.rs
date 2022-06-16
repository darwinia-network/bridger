use client_pangolin::component::PangolinClientComponent;
use client_pangoro::component::PangoroClientComponent;

use abstract_client_s2s::client::S2SClient;
use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangoro: ChainInfoConfig = bridge_config.pangoro;

    init_bridge(bridge, config_pangolin, config_pangoro).await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_pangolin: ChainInfoConfig,
    config_pangoro: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_pangolin = PangolinClientComponent::component(config_pangolin.into()).await?;
    let client_pangoro = PangoroClientComponent::component(config_pangoro.into()).await?;
    let hash = match bridge {
        BridgeName::PangolinToPangoro => {
            let initialization_data = client_pangolin.prepare_initialization_data().await?;
            let encoded = codec::Encode::encode(&initialization_data);
            client_pangoro
                .runtime()
                .tx()
                .bridge_pangolin_grandpa()
                .initialize(codec::Decode::decode(&mut &encoded[..])?)
                .sign_and_submit(client_pangoro.account().signer())
                .await?
        }
        BridgeName::PangoroToPangolin => {
            let initialization_data = client_pangoro.prepare_initialization_data().await?;
            let encoded = codec::Encode::encode(&initialization_data);
            client_pangolin
                .runtime()
                .tx()
                .bridge_pangoro_grandpa()
                .initialize(codec::Decode::decode(&mut &encoded[..])?)
                .sign_and_submit(client_pangolin.account().signer())
                .await?
        }
    };
    tracing::info!(
        target: "pangolin-pangoro",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
