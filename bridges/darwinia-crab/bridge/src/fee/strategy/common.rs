use relay_substrate_client::TransactionSignScheme;

use client_crab::api::CrabApi;
use client_crab::CrabChain;
use client_darwinia::api::DarwiniaApi;
use client_darwinia::DarwiniaChain;
use support_common::config::{Config, Names};

use crate::bridge::DarwiniaCrabConfig;
use crate::bridge::{ChainInfoConfig, RelayConfig};

#[derive(Clone)]
pub struct StrategyHelper {
    darwinia_api: DarwiniaApi,
    crab_api: CrabApi,
    darwinia_signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
    crab_signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
}

impl StrategyHelper {
    pub async fn new() -> color_eyre::Result<Self> {
        let bridge_config: DarwiniaCrabConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
        let config_darwinia: ChainInfoConfig = bridge_config.darwinia;
        let config_crab: ChainInfoConfig = bridge_config.crab;
        let config_relay: RelayConfig = bridge_config.relay;

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
    pub async fn reconnect_darwinia(&mut self) -> color_eyre::Result<()> {
        Ok(self.darwinia_api.reconnect().await?)
    }
    pub async fn reconnect_crab(&mut self) -> color_eyre::Result<()> {
        Ok(self.crab_api.reconnect().await?)
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
