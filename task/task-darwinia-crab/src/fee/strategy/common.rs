use relay_substrate_client::TransactionSignScheme;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_crab_s2s::api::CrabApi;
use component_crab_s2s::CrabChain;
use component_darwinia_s2s::api::DarwiniaApi;
use component_darwinia_s2s::DarwiniaChain;

use crate::config::{ChainInfoConfig, RelayConfig};
use crate::task::DarwiniaCrabTask;

#[derive(Clone)]
pub struct StrategyHelper {
    darwinia_api: DarwiniaApi,
    crab_api: CrabApi,
    darwinia_signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
    crab_signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
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

        let darwinia_signer = darwinia_chain.to_keypair::<DarwiniaChain>()?;
        let crab_signer = crab_chain.to_keypair::<CrabChain>()?;
        Ok(Self {
            darwinia_api: DarwiniaApi::new(darwinia_client),
            crab_api: CrabApi::new(crab_client),
            darwinia_signer,
            crab_signer,
        })
    }
}

impl StrategyHelper {
    pub fn darwinia_api_mut(&mut self) -> &mut DarwiniaApi {
        &mut self.darwinia_api
    }
    pub fn crab_api_mut(&mut self) -> &mut CrabApi {
        &mut self.crab_api
    }
}

impl StrategyHelper {
    pub fn darwinia_api(&self) -> &DarwiniaApi {
        &self.darwinia_api
    }
    pub fn crab_api(&self) -> &CrabApi {
        &self.crab_api
    }
    pub fn darwinia_signer(&self) -> &<DarwiniaChain as TransactionSignScheme>::AccountKeyPair {
        &self.darwinia_signer
    }
    pub fn crab_signer(&self) -> &<CrabChain as TransactionSignScheme>::AccountKeyPair {
        &self.crab_signer
    }
}
