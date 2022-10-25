use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use relay_e2e::error::{RelayError, RelayResult};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};
use client_pangoro::client::PangoroClient;
use relay_e2e::ecdsa::{
    ecdsa_scanner::{EcdsaScanType, EcdsaScanner as EcdsaScannerTrait},
    types::EcdsaSource,
};

#[derive(Debug)]
pub struct ECDSARelayService {
    _greet_collecting_message: Lifeline,
    _greet_collected_message: Lifeline,
    _greet_collecting_authorities: Lifeline,
    _greet_collected_authorities: Lifeline,
}

impl BridgeService for ECDSARelayService {}

impl Service for ECDSARelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(BridgeTask::name());
        let tracker_collecting_message =
            Tracker::new(microkv.clone(), "scan.pangoro.collecting-message");
        let tracker_collected_message =
            Tracker::new(microkv.clone(), "scan.pangoro.collected-message");
        let tracker_collecting_authorities =
            Tracker::new(microkv.clone(), "scan.pangoro.collecting-authorities");
        let tracker_collected_authorities =
            Tracker::new(microkv, "scan.pangoro.collected-authorities");
        let _greet_collecting_message =
            Self::try_task("pangoro-to-goerli-ecdsa-collecting-message", async move {
                EcdsaScanner
                    .start(
                        tracker_collecting_message.clone(),
                        EcdsaScanType::CollectingMessage,
                    )
                    .await;
                Ok(())
            });
        let _greet_collected_message =
            Self::try_task("pangoro-to-goerli-ecdsa-collected-message", async move {
                EcdsaScanner
                    .start(
                        tracker_collected_message.clone(),
                        EcdsaScanType::CollectedMessage,
                    )
                    .await;
                Ok(())
            });
        let _greet_collecting_authorities = Self::try_task(
            "pangoro-to-goerli-ecdsa-collecting-authorities",
            async move {
                EcdsaScanner
                    .start(
                        tracker_collecting_authorities.clone(),
                        EcdsaScanType::CollectingAuthority,
                    )
                    .await;
                Ok(())
            },
        );
        let _greet_collected_authorities = Self::try_task(
            "pangoro-to-goerli-ecdsa-collected-authorities",
            async move {
                EcdsaScanner
                    .start(
                        tracker_collected_authorities.clone(),
                        EcdsaScanType::CollectedAuthority,
                    )
                    .await;
                Ok(())
            },
        );
        Ok(Self {
            _greet_collecting_message,
            _greet_collected_message,
            _greet_collecting_authorities,
            _greet_collected_authorities,
        })
    }
}

#[derive(Debug, Clone)]
pub struct EcdsaScanner;

#[async_trait::async_trait]
impl EcdsaScannerTrait<PangoroClient> for EcdsaScanner {
    async fn get_ecdsa_source(&self) -> RelayResult<EcdsaSource<PangoroClient>> {
        let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let subquery = config.index.to_pangoro_subquery();
        let client_darwinia_web3 = config
            .pangoro_evm
            .to_web3_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let client_eth_web3 = config
            .goerli
            .to_web3_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let client_darwinia_substrate = config
            .pangoro_substrate
            .to_substrate_client()
            .await
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let client_posa = config
            .goerli
            .to_posa_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let darwinia_evm_account = config.pangoro_evm.to_fast_ethereum_account();
        let ethereum_account = config.goerli.to_ethereum_account();
        let minimal_interval = config.general.header_relay_minimum_interval;
        Ok(EcdsaSource {
            block: None,
            client_darwinia_web3,
            client_eth_web3,
            subquery,
            client_posa,
            client_darwinia_substrate,
            ethereum_account,
            darwinia_evm_account,
            minimal_interval,
        })
    }
}
