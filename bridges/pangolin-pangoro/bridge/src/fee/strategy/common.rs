use relay_substrate_client::TransactionSignScheme;

use client_pangolin::api::PangolinApi;
use client_pangolin::PangolinChain;
use client_pangoro::api::PangoroApi;
use client_pangoro::PangoroChain;
use support_common::config::{Config, Names};

use crate::bridge::{ChainInfoConfig, RelayConfig};
use crate::bridge::{PangolinPangoroConfig, PangolinPangoroTask};

#[derive(Clone)]
pub struct StrategyHelper {
    pangolin_api: PangolinApi,
    pangoro_api: PangoroApi,
    pangolin_signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
    pangoro_signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
}

impl StrategyHelper {
    pub async fn new() -> color_eyre::Result<Self> {
        let bridge_config: PangolinPangoroConfig = Config::restore(Names::BridgePangolinPangoro)?;
        let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
        let config_pangoro: ChainInfoConfig = bridge_config.pangoro;
        let config_relay: RelayConfig = bridge_config.relay;

        let (pangolin_chain, pangoro_chain) = (
            config_pangolin
                .to_chain_info_with_expect_signer(config_relay.signer_pangolin.clone())?,
            config_pangoro.to_chain_info_with_expect_signer(config_relay.signer_pangoro.clone())?,
        );

        let pangolin_client = pangolin_chain
            .to_substrate_relay_chain::<PangolinChain>()
            .await?;
        let pangoro_client = pangoro_chain
            .to_substrate_relay_chain::<PangoroChain>()
            .await?;

        let pangolin_signer = pangolin_chain.to_keypair::<PangolinChain>()?;
        let pangoro_signer = pangoro_chain.to_keypair::<PangoroChain>()?;
        Ok(Self {
            pangolin_api: PangolinApi::new(pangolin_client),
            pangoro_api: PangoroApi::new(pangoro_client),
            pangolin_signer,
            pangoro_signer,
        })
    }
}

impl StrategyHelper {
    pub async fn reconnect_pangolin(&mut self) -> color_eyre::Result<()> {
        Ok(self.pangolin_api.reconnect().await?)
    }
    pub async fn reconnect_pangoro(&mut self) -> color_eyre::Result<()> {
        Ok(self.pangoro_api.reconnect().await?)
    }
}

impl StrategyHelper {
    pub fn pangolin_api(&self) -> &PangolinApi {
        &self.pangolin_api
    }
    pub fn pangoro_api(&self) -> &PangoroApi {
        &self.pangoro_api
    }
    pub fn pangolin_signer(&self) -> &<PangolinChain as TransactionSignScheme>::AccountKeyPair {
        &self.pangolin_signer
    }
    pub fn pangoro_signer(&self) -> &<PangoroChain as TransactionSignScheme>::AccountKeyPair {
        &self.pangoro_signer
    }
}
