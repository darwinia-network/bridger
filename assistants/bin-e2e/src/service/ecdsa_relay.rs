use std::marker::PhantomData;

use bridge_e2e_traits::client::EcdsaClient;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use relay_e2e::ecdsa::ecdsa_scanner::EcdsaScanner;
use relay_e2e::error::{RelayError, RelayResult};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::BridgeBus;
use crate::config::BridgeConfig;
use relay_e2e::ecdsa::{ecdsa_scanner::EcdsaScanType, types::EcdsaSource};

#[derive(Debug)]
pub struct ECDSARelayService<T: EcdsaClient> {
    _greet_collecting_message: Lifeline,
    _greet_collected_message: Lifeline,
    _greet_collecting_authorities: Lifeline,
    _greet_collected_authorities: Lifeline,
    _ecdsa: PhantomData<T>,
}

impl<T: EcdsaClient> BridgeService for ECDSARelayService<T> {}

impl<T: EcdsaClient> Service for ECDSARelayService<T> {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<T> = bus.storage().clone_resource()?;
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(bridge_config.name.clone());
        let tracker_collecting_message =
            Tracker::new(microkv.clone(), "scan.darwinia.collecting-message");
        let tracker_collected_message =
            Tracker::new(microkv.clone(), "scan.darwinia.collected-message");
        let tracker_collecting_authorities =
            Tracker::new(microkv.clone(), "scan.darwinia.collecting-authorities");
        let tracker_collected_authorities =
            Tracker::new(microkv, "scan.darwinia.collected-authorities");
        let ecdsa_source = Self::get_ecdsa_source(bridge_config.clone())?;
        let _greet_collecting_message =
            Self::try_task("darwinia-to-eth-ecdsa-collecting-message", async move {
                ecdsa_source
                    .start(
                        tracker_collecting_message.clone(),
                        EcdsaScanType::CollectingMessage,
                    )
                    .await;
                Ok(())
            });
        let ecdsa_source = Self::get_ecdsa_source(bridge_config.clone())?;
        let _greet_collected_message =
            Self::try_task("darwinia-to-eth-ecdsa-collected-message", async move {
                ecdsa_source
                    .start(
                        tracker_collected_message.clone(),
                        EcdsaScanType::CollectedMessage,
                    )
                    .await;
                Ok(())
            });
        let ecdsa_source = Self::get_ecdsa_source(bridge_config.clone())?;
        let _greet_collecting_authorities =
            Self::try_task("darwinia-to-eth-ecdsa-collecting-authorities", async move {
                ecdsa_source
                    .start(
                        tracker_collecting_authorities.clone(),
                        EcdsaScanType::CollectingAuthority,
                    )
                    .await;
                Ok(())
            });
        let ecdsa_source = Self::get_ecdsa_source(bridge_config.clone())?;
        let _greet_collected_authorities =
            Self::try_task("darwinia-to-eth-ecdsa-collected-authorities", async move {
                ecdsa_source
                    .start(
                        tracker_collected_authorities.clone(),
                        EcdsaScanType::CollectedAuthority,
                    )
                    .await;
                Ok(())
            });
        Ok(Self {
            _greet_collecting_message,
            _greet_collected_message,
            _greet_collecting_authorities,
            _greet_collected_authorities,
            _ecdsa: Default::default(),
        })
    }
}

impl<T: EcdsaClient> ECDSARelayService<T> {
    fn get_ecdsa_source(config: BridgeConfig<T>) -> RelayResult<EcdsaSource<T>> {
        let subquery = config.substrate_index;
        let client_darwinia_web3 = config
            .darwinia_evm
            .to_web3_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let client_eth_web3 = config
            .ethereum
            .to_web3_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let client_darwinia_substrate = config.substrate_client;
        let client_posa = config
            .ethereum
            .to_posa_client()
            .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let darwinia_evm_account = config.darwinia_evm.to_ethereum_account();
        let ethereum_account = config.ethereum.to_ethereum_account();
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
