use bridge_traits::bridge::config::Config;
use lifeline::{Lifeline, Service, Task};
use relay_substrate_client::TransactionSignScheme;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;
use component_pangoro_s2s::api::PangoroApi;
use component_pangoro_s2s::PangoroChain;

use crate::bus::PangolinPangoroBus;
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::task::PangolinPangoroTask;

#[derive(Debug)]
pub struct UpdateFeeService {
    _greet: Lifeline,
}

impl BridgeService for UpdateFeeService {}

impl Service for UpdateFeeService {
    type Bus = PangolinPangoroBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangolin")?;
        let config_pangoro: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangoro")?;
        let config_relay: RelayConfig = Config::restore(PangolinPangoroTask::NAME)?;

        let _greet = Self::try_task(
            &format!("{}-update-fee", PangolinPangoroTask::NAME),
            async move {
                let (pangolin_chain, pangoro_chain) = (
                    config_pangolin
                        .to_chain_info_with_expect_signer(config_relay.signer_pangolin.clone())?,
                    config_pangoro
                        .to_chain_info_with_expect_signer(config_relay.signer_pangoro.clone())?,
                );

                let pangolin_client = pangolin_chain
                    .to_substrate_relay_chain::<PangolinChain>()
                    .await?;
                let pangoro_client = pangoro_chain
                    .to_substrate_relay_chain::<PangoroChain>()
                    .await?;

                let pangolin_sign = pangolin_chain.to_keypair::<PangolinChain>()?;
                let pangoro_sign = pangoro_chain.to_keypair::<PangoroChain>()?;
                let runner = UpdateFeeRunner::new(
                    PangolinApi::new(pangolin_client),
                    PangoroApi::new(pangoro_client),
                    pangolin_sign,
                    pangoro_sign,
                );
                runner.handle().await?;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct UpdateFeeRunner {
    pangolin_api: PangolinApi,
    pangoro_api: PangoroApi,
    pangolin_signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
    pangoro_signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
}

impl UpdateFeeRunner {
    pub fn new(
        pangolin_api: PangolinApi,
        pangoro_api: PangoroApi,
        pangolin_signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        pangoro_signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
    ) -> Self {
        Self {
            pangolin_api,
            pangoro_api,
            pangolin_signer,
            pangoro_signer,
        }
    }
}

impl UpdateFeeRunner {
    pub async fn handle(&self) -> anyhow::Result<()> {
        // cost = confirmation_transaction_cost(source chain)
        // loop {
        //     //
        // }
        Ok(())
    }
}
