use std::{marker::PhantomData, str::FromStr};

use crate::bridge::BridgeBus;
use crate::config::BridgeConfig;
use bridge_e2e_traits::client::EcdsaClient;
use client_beacon::client::BeaconApiClient;
use lifeline::{dyn_bus::DynBus, Lifeline, Service, Task};
use relay_e2e::header::{
    common::EthLightClient, eth_sync_committee_relay::SyncCommitteeRelayRunner,
};

use support_toolkit::timecount::TimeCount;
use support_lifeline::service::BridgeService;
use web3::types::{Address, U256};

#[derive(Debug)]
pub struct SyncCommitteeUpdateService<T: EcdsaClient> {
    _greet: Lifeline,
    _ecdsa: PhantomData<T>,
}

impl<T: EcdsaClient> BridgeService for SyncCommitteeUpdateService<T> {}

impl<T: EcdsaClient> Service for SyncCommitteeUpdateService<T> {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<T> = bus.storage().clone_resource()?;
        let _greet = Self::try_task("sync-committee-update-eth-to-darwinia", async move {
            let mut timecount = TimeCount::new();
            while let Err(error) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "substrate-eth",
                    "Failed to start sync committee update relay service, restart after some seconds: {:?}",
                    error
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "substrate-eth",
                        "[sync-committee-relay] many errors occurred, wait {} seconds",
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self {
            _greet,
            _ecdsa: Default::default(),
        })
    }
}

impl<T: EcdsaClient> SyncCommitteeUpdateService<T> {
    async fn start(config: BridgeConfig<T>) -> color_eyre::Result<()> {
        let darwinia_client = EthLightClient::new(
            &config.darwinia_evm.endpoint,
            Address::from_str(&config.darwinia_evm.contract_address)?,
            &config.darwinia_evm.private_key,
            U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
        )?;
        let eth_client = BeaconApiClient::new(&config.beacon.endpoint, config.beacon.api_supplier)?;
        let mut update_manager = SyncCommitteeRelayRunner {
            eth_light_client: darwinia_client,
            beacon_api_client: eth_client,
        };

        update_manager.start().await?;
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        Ok(())
    }
}
