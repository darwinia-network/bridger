use relay_substrate_client::TransactionSignScheme;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;
use component_pangoro_s2s::api::PangoroApi;
use component_pangoro_s2s::PangoroChain;

use crate::config::{ChainInfoConfig, RelayConfig};
use crate::task::PangolinPangoroTask;

#[derive(Clone)]
pub struct StrategyHelper {
    pangolin_api: PangolinApi,
    pangoro_api: PangoroApi,
    pangolin_signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
    pangoro_signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
}

impl StrategyHelper {
    pub async fn new() -> anyhow::Result<Self> {
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(PangolinPangoroTask::NAME, "pangolin")?;
        let config_pangoro: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(PangolinPangoroTask::NAME, "pangoro")?;
        let config_relay: RelayConfig = Config::restore_unwrap(PangolinPangoroTask::NAME)?;

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
    pub fn pangolin_api_mut(&mut self) -> &mut PangolinApi {
        &mut self.pangolin_api
    }
    pub fn pangoro_api_mut(&mut self) -> &mut PangoroApi {
        &mut self.pangoro_api
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
