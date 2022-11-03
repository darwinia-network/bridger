use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::RelaychainHeaderRunner;
use relay_s2s::types::RelaychainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PolkadotToDarwiniaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PolkadotToDarwiniaHeaderRelayService {}

impl Service for PolkadotToDarwiniaHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("polkadot-to-darwinia-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "darwinia-darwiniaparachain",
                    "[header-relay] [polkadot-to-darwinia] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "darwinia-darwiniaparachain",
                    "[header-relay] [polkadot-to-darwinia] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-darwiniaparachain",
        "[header-polkadot-to-darwinia] [polkadot-to-darwinia] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;

    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;
    let client_polkadot = bridge_config.polkadot.to_polkadot_client().await?;

    let config_index = bridge_config.index;
    let subquery_polkadot = config_index.to_polkadot_subquery();
    let subquery_darwinia_parachain = config_index.to_darwinia_parachain_subquery();

    let input = RelaychainHeaderInput {
        client_relaychain: client_polkadot,
        client_solochain: client_darwinia,
        subquery_relaychain: subquery_polkadot,
        subquery_parachain: subquery_darwinia_parachain,
        index_origin_type: OriginType::BridgeDarwinia,
        enable_mandatory: bridge_config.relay.enable_mandatory,
    };
    let runner = RelaychainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
