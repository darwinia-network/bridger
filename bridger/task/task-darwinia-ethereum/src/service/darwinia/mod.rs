mod darwinia_tracker;
pub use darwinia_tracker::DarwiniaBlockTracker;
use std::time::Duration;
use tokio::time::sleep;

use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Sender, Task};
use microkv::MicroKV;
use postage::broadcast;
use web3::{
    transports::http::Http,
    types::{Log, H160, H256},
    Web3,
};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::darwinia::client::Darwinia;
use component_ethereum::web3::Web3Component;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::{SubstrateEthereumConfig, DarwiniaEthereumConfig};
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage, ToDarwiniaLinkedMessage, ToRedeemMessage, ToRelayMessage, ToDarwiniaMessage, Extrinsic, ToExtrinsicsMessage};
use crate::task::DarwiniaEthereumTask;
use component_darwinia_subxt::darwinia::runtime::DarwiniaRuntime;
use substrate_subxt::system::System;

use crate::error::{Result, Error, BizError};
use component_darwinia_subxt::events::EventInfo;
use std::collections::HashMap;
use crate::ethereum::Ethereum;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::
    to_ethereum::{
        Darwinia2Ethereum, Account as ToEthereumAccount
    }
;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_ethereum::config::{Web3Config, EthereumConfig};

#[derive(Debug)]
pub struct DarwiniaService {
    _greet: Lifeline,
}

impl BridgeService for DarwiniaService {}

impl lifeline::Service for DarwiniaService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToDarwiniaMessage>()?;
        let mut sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Components
        let component_web3 = Web3Component::restore::<DarwiniaEthereumTask>()?;
        let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;

        // Config
        let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
        let config_ethereum: EthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
        let config_web3: Web3Config = Config::restore(DarwiniaEthereumTask::NAME)?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-darwinia-scan", DarwiniaEthereumTask::NAME),
            async move {
                debug!(target: DarwiniaEthereumTask::NAME, "hello darwinia-scan");
                let mut delayed_extrinsics: HashMap<u32, Extrinsic> = HashMap::new();

                let microkv = state.microkv();

                // Darwinia client & account
                let darwinia = component_darwinia_subxt.component().await?;
                let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
                let account = DarwiniaAccount::new(config_darwinia.endpoint, config_darwinia.relayer_real_account);
                let account = ToEthereumAccount::new(account.clone(), config_darwinia.ecdsa_authority_private_key, config_web3.endpoint);

                // Ethereum client
                let web3 = component_web3.component().await?;
                let ethereum = Ethereum::new(web3, config_ethereum.relayer_relay_contract_address, config_ethereum.relayer_private_key, config_ethereum.relayer_beneficiary_darwinia_account)?;

                let spec_name = darwinia.runtime_version().await?;
                let scan_from = microkv.get("darwinia_scan_from")?.unwrap_or(032);

                let mut runner = DarwiniaServiceRunner {
                    darwinia2ethereum,
                    account,
                    ethereum,
                    sender_to_extrinsics,
                    delayed_extrinsics,
                    spec_name,
                    scan_from,
                };
                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToDarwiniaMessage::Start => {
                            runner.start(microkv).await?;
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct DarwiniaServiceRunner {
    darwinia2ethereum: Darwinia2Ethereum,
    account: ToEthereumAccount,
    ethereum: Ethereum,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    delayed_extrinsics: HashMap<u32, Extrinsic>,
    spec_name: String,
    scan_from: u32,
}

impl DarwiniaServiceRunner {

    /// start
    pub async fn start(
        &mut self,
        microkv: &MicroKV,
    ) -> Result<()> {
        let mut tracker =
            DarwiniaBlockTracker::new(self.darwinia2ethereum.darwinia.clone(), self.scan_from);
        info!("✨ SERVICE STARTED: SUBSCRIBE");
        loop {
            let header = tracker.next_block().await?;

            // debug
            trace!("Darwinia block {}", header.number);

            // handle the 'mmr root sign and send extrinsics' only block height reached
            if let Err(err) = self.handle_delayed_extrinsics(&header).await {
                error!(
					"An error occurred while processing the delayed extrinsics: {:?}",
					err
				);
                // Prevent too fast refresh errors
                sleep(Duration::from_secs(30)).await;
            }

            // handle events of the block
            let hash = header.hash();
            let events = self
                .darwinia2ethereum
                .darwinia
                .get_events_from_block_hash(hash)
                .await
                .map_err(|err| err.into());
            if let Err(err) = self.handle_events(&header, events).await {
                if let Some(Error::RuntimeUpdated) = err.downcast_ref() {
                    microkv.put("last-tracked-darwinia-block", &(header.number));
                    return Err(err);
                } else {
                    error!(
						"An error occurred while processing the events of block {}: {:?}",
						header.number, err
					);
                    sleep(Duration::from_secs(30)).await;
                }
            } else {
                microkv.put("last-tracked-darwinia-block", &(header.number));
            }

        }
    }

    async fn handle_delayed_extrinsics(
        &mut self,
        header: &<DarwiniaRuntime as System>::Header,
    ) -> Result<()> {
        let cloned = self.delayed_extrinsics.clone();
        for (delayed_to, delayed_ex) in cloned.iter() {
            if header.number >= *delayed_to
                && self
                .darwinia2ethereum
                .need_to_sign_mmr_root_of(&self.account, *delayed_to, Some(header.number))
                .await?
            {
                self.sender_to_extrinsics.send(ToExtrinsicsMessage::Extrinsic(delayed_ex.clone())).await?;
                self.delayed_extrinsics.remove(&delayed_to);
            }
        }
        Ok(())
    }

    async fn handle_events(
        &mut self,
        header: &<DarwiniaRuntime as System>::Header,
        events: Result<Vec<EventInfo<DarwiniaRuntime>>>,
    ) -> Result<()> {
        for event in events? {
            self.handle_event(header, event).await?;
        }
        Ok(())
    }

    async fn handle_event(
        &mut self,
        header: &<DarwiniaRuntime as System>::Header,
        event: EventInfo<DarwiniaRuntime>,
    ) -> Result<()> {
        //todo
        //if module != "System" {
        //trace!(">> Event - {}::{}", module, variant);
        //}
        let block = Some(header.number);
        match event {
            EventInfo::RuntimeUpdatedEvent(_) => {
                return Err(Error::RuntimeUpdated.into());
            }
            // call ethereum_relay_authorities.request_authority and then sudo call
            // EthereumRelayAuthorities.add_authority will emit the event
            EventInfo::ScheduleAuthoritiesChangeEvent(event) => {
                if self
                    .darwinia2ethereum
                    .is_authority(block, &self.account)
                    .await? && self
                    .darwinia2ethereum
                    .need_to_sign_authorities(block, &self.account, event.message)
                    .await?
                {
                    let ex = Extrinsic::SignAndSendAuthorities(event.message);
                    self.sender_to_extrinsics.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;
                }
            }
            // authority set changed will emit this event
            EventInfo::AuthoritiesChangeSignedEvent(event) => {
                let current_term = self.darwinia2ethereum.get_current_authority_term().await?;
                if event.term == current_term {
                    let message = Darwinia2Ethereum::construct_authorities_message(
                        self.spec_name.clone(),
                        event.term,
                        event.new_authorities,
                    );
                    let signatures = event
                        .signatures
                        .iter()
                        .map(|s| s.1.clone())
                        .collect::<Vec<_>>();
                    let tx_hash = self
                        .ethereum
                        .submit_authorities_set(message, signatures)
                        .await?;
                    info!("Submit authorities to ethereum with tx: {}", tx_hash);
                }
            }
            // call ethereum_backing.lock will emit the event
            EventInfo::ScheduleMMRRootEvent(event) => {
                if self
                    .darwinia2ethereum
                    .is_authority(block, &self.account)
                    .await?
                {
                    info!("{}", event);
                    let ex = Extrinsic::SignAndSendMmrRoot(event.block_number);
                    self.delayed_extrinsics.insert(event.block_number, ex);
                }
            }
            _ => {}
        }
        Ok(())
    }
}


