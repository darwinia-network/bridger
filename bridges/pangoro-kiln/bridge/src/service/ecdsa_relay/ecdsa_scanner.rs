use microkv::namespace::NamespaceMicroKV;
use support_common::config::{Config, Names};
use support_tracker::Tracker;

use crate::bridge::BridgeConfig;
use crate::service::ecdsa_relay::collected_enough_authorities_change_signatures::CollectedEnoughAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collected_enough_new_message_root_signatures::CollectedEnoughNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::collecting_authorities_change_signatures::CollectingAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collecting_new_message_root_signatures::CollectingNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct EcdsaScanner;

impl EcdsaScanner {
    pub async fn start(&self, microkv: NamespaceMicroKV, tracker: Tracker) {
        while let Err(err) = self.run(microkv.clone(), tracker.clone()).await {
            tracing::error!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] An error occurred while processing the extrinsics: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run(&self, microkv: NamespaceMicroKV, tracker: Tracker) -> color_eyre::Result<()> {
        let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
        let subquery = config.index.to_pangoro_subquery();
        let client_pangoro_web3 = config.pangoro_evm.to_web3_client()?;
        let client_pangoro_substrate = config.pangoro_substrate.to_substrate_client().await?;
        let client_posa = config.kiln.to_posa_client()?;
        let pangoro_evm_account = config.pangoro_evm.to_fast_ethereum_account();
        let ethereum_account = config.kiln.to_ethereum_account();
        let mut source = EcdsaSource {
            block: None,
            subquery,
            client_pangoro_web3,
            client_pangoro_substrate,
            client_posa,
            pangoro_evm_account,
            ethereum_account,
        };

        loop {
            let from = tracker.current().await?;
            tracing::info!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] Track pangoro scan block: {} ",
                from,
            );
            source.block = Some(from as u32);

            let runner = CollectingAuthoritiesChangeSignaturesRunner::new(source.clone());
            runner.start().await?;

            let runner = CollectingNewMessageRootSignaturesRunner::new(source.clone());
            runner.start().await?;

            let runner = CollectedEnoughAuthoritiesChangeSignaturesRunner::new(source.clone());
            runner.start().await?;

            let runner = CollectedEnoughNewMessageRootSignaturesRunner::new(source.clone());
            runner.start().await?;

            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        }
    }
}
