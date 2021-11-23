use relay_substrate_client::TransactionSignScheme;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_crab_s2s::api::PangoroApi;
use component_crab_s2s::CrabChain;
use component_darwinia_s2s::api::PangolinApi;
use component_darwinia_s2s::DarwiniaChain;

use crate::config::{ChainInfoConfig, RelayConfig};
use crate::task::DarwiniaCrabTask;

#[derive(Clone)]
pub struct StrategyHelper {
    pangolin_api: PangolinApi,
    pangoro_api: PangoroApi,
    pangolin_signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
    pangoro_signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
}

impl StrategyHelper {
    pub async fn new() -> anyhow::Result<Self> {
        let config_darwinia: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(DarwiniaCrabTask::NAME, "darwinia")?;
        let config_crab: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(DarwiniaCrabTask::NAME, "crab")?;
        let config_relay: RelayConfig = Config::restore_unwrap(DarwiniaCrabTask::NAME)?;

        let (darwinia_chain, crab_chain) = (
            config_darwinia
                .to_chain_info_with_expect_signer(config_relay.signer_darwinia.clone())?,
            config_crab.to_chain_info_with_expect_signer(config_relay.signer_crab.clone())?,
        );

        let darwinia_client = darwinia_chain
            .to_substrate_relay_chain::<DarwiniaChain>()
            .await?;
        let crab_client = crab_chain.to_substrate_relay_chain::<CrabChain>().await?;

        let pangolin_signer = darwinia_chain.to_keypair::<DarwiniaChain>()?;
        let pangoro_signer = crab_chain.to_keypair::<CrabChain>()?;
        Ok(Self {
            pangolin_api: PangolinApi::new(darwinia_client),
            pangoro_api: PangoroApi::new(crab_client),
            pangolin_signer,
            pangoro_signer,
        })
    }
}

impl StrategyHelper {
    pub fn pangolin_api(&self) -> &PangolinApi {
        &self.pangolin_api
    }
    pub fn pangoro_api(&self) -> &PangoroApi {
        &self.pangoro_api
    }
    pub fn pangolin_signer(&self) -> &<DarwiniaChain as TransactionSignScheme>::AccountKeyPair {
        &self.pangolin_signer
    }
    pub fn pangoro_signer(&self) -> &<CrabChain as TransactionSignScheme>::AccountKeyPair {
        &self.pangoro_signer
    }
}
