use lifeline::{Lifeline, Service, Task};

use relay_s2s::header::ParaHeaderRunner;
use relay_s2s::types::ParaHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct ParaHeadRelayService {
    _greet: Lifeline,
}

impl BridgeService for ParaHeadRelayService {}

impl Service for ParaHeadRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
       let _greet = Self::try_task("para-head-relay-service", async move {
           while let Err(e) = start().await {
               tracing::error!(
                   target: "darwinia-darwiniaparachain",
                   "[header-relay] [para-head-to-darwinia] An error occurred for header relay {:?}",
                   e,
               );
               tokio::time::sleep(std::time::Duration::from_secs(5)).await;
               tracing::info!(
                   target: "darwinia-darwiniaparachain",
                   "[header-relay] [para-head-to-darwinia] Try to restart header relay service.",
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
       "[header-para-head-to-darwinia] [para-head-to-darwinia] SERVICE RESTARTING..."
   );
   let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
   let config_relay = bridge_config.relay;

   let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;
   let client_polkadot = bridge_config.polkadot.to_polkadot_client().await?;

   let input = ParaHeaderInput {
       client_relaychain: client_polkadot,
       client_solochain: client_darwinia,
       para_id: config_relay.para_id,
   };
   let runner = ParaHeaderRunner::new(input);
    Ok(runner.start().await?)
}
