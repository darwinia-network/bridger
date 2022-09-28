use support_common::config::{Config, Names};
use support_tracker::Tracker;

use crate::bridge::BridgeConfig;
use crate::service::ecdsa_relay::collected_enough_authorities_change_signatures::CollectedEnoughAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collected_enough_new_message_root_signatures::CollectedEnoughNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::collecting_authorities_change_signatures::CollectingAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collecting_new_message_root_signatures::CollectingNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::types::EcdsaSource;

#[derive(Clone, Copy, Debug)]
pub enum EcdsaScanType {
    CollectingMessage,
    CollectedMessage,
    CollectingAuthority,
    CollectedAuthority,
}

pub struct EcdsaScanner;

impl EcdsaScanner {
    pub async fn start(&self, tracker: Tracker, scan_type: EcdsaScanType) {
        while let Err(err) = self.run(tracker.clone(), scan_type).await {
            tracing::error!(
                target: "darwinia-ethereum",
                "[darwinia] [ecdsa] An error occurred while processing the extrinsics: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run(&self, tracker: Tracker, scan_type: EcdsaScanType) -> color_eyre::Result<()> {
        let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
        let subquery = config.index.to_darwinia_subquery();
        let client_darwinia_web3 = config.darwinia_evm.to_web3_client()?;
        let client_ethereum_web3 = config.ethereum.to_web3_client()?;
        let client_darwinia_substrate = config.darwinia_substrate.to_substrate_client().await?;
        let client_posa = config.ethereum.to_posa_client()?;
        let darwinia_evm_account = config.darwinia_evm.to_fast_ethereum_account();
        let ethereum_account = config.ethereum.to_ethereum_account();
        let mut source = EcdsaSource {
            block: None,
            subquery,
            client_darwinia_web3,
            client_ethereum_web3,
            client_darwinia_substrate,
            client_posa,
            darwinia_evm_account,
            ethereum_account,
        };

        loop {
            let from = tracker.current().await?;
            tracing::info!(
                target: "darwinia-ethereum",
                "[darwinia] [ecdsa] Track darwinia scan block: {} ",
                from,
            );
            source.block = Some(from as u32);

            let finished_block = match scan_type {
                EcdsaScanType::CollectingMessage => {
                    let runner = CollectingNewMessageRootSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
                EcdsaScanType::CollectedMessage => {
                    let mut runner = CollectedEnoughNewMessageRootSignaturesRunner::new(
                        source.clone(),
                        config.general.header_relay_minimum_interval,
                    );
                    runner.start().await?
                }
                EcdsaScanType::CollectingAuthority => {
                    let runner = CollectingAuthoritiesChangeSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
                EcdsaScanType::CollectedAuthority => {
                    let runner =
                        CollectedEnoughAuthoritiesChangeSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
            };
            if finished_block.is_some() {
                tracker.finish(finished_block.unwrap() as usize)?;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
}
