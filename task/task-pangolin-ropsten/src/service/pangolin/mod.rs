use std::collections::HashMap;

use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Sender, Task};
use postage::broadcast;
use substrate_subxt::system::System;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::config::Web3Config;
use component_ethereum::ethereum::client::EthereumClient;
use component_ethereum::ethereum::EthereumComponent;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::config::DarwiniaSubxtConfig;
use component_pangolin_subxt::darwinia::runtime::DarwiniaRuntime;
use component_pangolin_subxt::events::EventInfo;
use component_pangolin_subxt::to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum};
use component_state::state::BridgeState;
use pangolin_tracker::PangolinBlockTracker;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::error::{Error, Result};
use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::task::PangolinRopstenTask;

mod pangolin_tracker;

#[derive(Debug)]
pub struct PangolinService {
    _greet: Lifeline,
}

impl BridgeService for PangolinService {}

impl lifeline::Service for PangolinService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let state = bus.storage().clone_resource::<BridgeState>()?;

        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker = Tracker::new(microkv, "scan.pangolin");

        let _greet = Self::try_task(
            &format!("{}-service-pangolin-scan", PangolinRopstenTask::NAME),
            async move {
                start(sender_to_extrinsics.clone(), tracker.clone()).await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>, tracker: Tracker) {
    while let Err(e) = run(sender_to_extrinsics.clone(), tracker.clone()).await {
        let secs = 10;
        error!(
            target: PangolinRopstenTask::NAME,
            "pangolin err {:#?}, wait {} seconds try again", e, secs
        );
        tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
    }
}

async fn run(
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    tracker: Tracker,
) -> anyhow::Result<()> {
    info!(
        target: PangolinRopstenTask::NAME,
        "PANGOLIN SCAN SERVICE RESTARTING..."
    );

    let delayed_extrinsics: HashMap<u32, Extrinsic> = HashMap::new();

    // Config
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(PangolinRopstenTask::NAME)?;
    let config_web3: Web3Config = Config::restore(PangolinRopstenTask::NAME)?;

    // Components
    let component_ethereum = EthereumComponent::restore::<PangolinRopstenTask>()?;
    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;

    // Darwinia client & account
    let darwinia = component_pangolin_subxt.component().await?;
    let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
    let account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );
    let account = ToEthereumAccount::new(
        account.clone(),
        config_darwinia.ecdsa_authority_private_key,
        config_web3.endpoint,
    );

    let ethereum = component_ethereum.component().await?;

    let spec_name = darwinia.runtime_version().await?;

    info!(
        target: PangolinRopstenTask::NAME,
        "✨ SERVICE STARTED: ROPSTEN <> PANGOLIN PANGOLIN SUBSCRIBE"
    );

    let mut runner = DarwiniaServiceRunner {
        darwinia2ethereum,
        account,
        ethereum,
        sender_to_extrinsics: sender_to_extrinsics.clone(),
        delayed_extrinsics,
        spec_name,
    };
    runner.start(tracker).await
}

struct DarwiniaServiceRunner {
    darwinia2ethereum: Darwinia2Ethereum,
    account: ToEthereumAccount,
    ethereum: EthereumClient,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    delayed_extrinsics: HashMap<u32, Extrinsic>,
    spec_name: String,
}

impl DarwiniaServiceRunner {
    /// start
    pub async fn start(&mut self, tracker_raw: Tracker) -> Result<()> {
        let tracker_darwinia =
            PangolinBlockTracker::new(self.darwinia2ethereum.darwinia.clone(), tracker_raw.clone());
        let mut retry_times = 0;
        loop {
            let header = tracker_darwinia.next_block().await?;

            // debug
            if header.number % 100 == 0 {
                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "Darwinia block {}",
                    header.number
                );
            }

            // handle the 'mmr root sign and send extrinsics' only block height reached
            if let Err(err) = self.handle_delayed_extrinsics(&header).await {
                log::error!(
                    target: PangolinRopstenTask::NAME,
                    "An error occurred while processing the delayed extrinsics: {:?}",
                    err
                );
                // Prevent too fast refresh errors
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            }

            // handle events of the block
            let hash = header.hash();
            let events = self
                .darwinia2ethereum
                .darwinia
                .get_events_from_block_hash(hash)
                .await
                .map_err(|err| err.into());

            // process events
            if let Err(err) = self.handle_events(&header, events).await {
                log::error!(
                    target: PangolinRopstenTask::NAME,
                    "An error occurred while processing the events of block {}: {:?}",
                    header.number,
                    err
                );

                if let Some(Error::RuntimeUpdated) = err.downcast_ref() {
                    // todo: write log
                    tracker_raw.finish(header.number as usize)?;
                    return Err(err);
                }

                let err_msg = format!("{:?}", err).to_lowercase();
                if err_msg.contains("type size unavailable") {
                    // todo: write log
                }

                if retry_times > 10 {
                    // todo: write log
                    log::error!(
                        target: PangolinRopstenTask::NAME,
                        "Retry {} times still failed: {}",
                        retry_times,
                        header.number
                    );
                    retry_times = 0;
                    continue;
                }
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                retry_times += 1;
                continue;
            }
            tracker_raw.finish(header.number as usize)?;
            retry_times = 0;
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
                self.sender_to_extrinsics
                    .send(ToExtrinsicsMessage::Extrinsic(delayed_ex.clone()))
                    .await?;
                self.delayed_extrinsics.remove(delayed_to);
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
        let block = Some(header.number);
        match event {
            EventInfo::RuntimeUpdatedEvent(_) => {
                return Err(Error::RuntimeUpdated.into());
            }
            // call ethereum_relay_authorities.request_authority and then sudo call
            // EthereumRelayAuthorities.add_authority will emit the event
            EventInfo::ScheduleAuthoritiesChangeEvent(event) => {
                info!(
                    target: PangolinRopstenTask::NAME,
                    "find schedule authorities change event, number {:?}", header.number
                );
                if self
                    .darwinia2ethereum
                    .is_authority(block, &self.account)
                    .await?
                    && self
                        .darwinia2ethereum
                        .need_to_sign_authorities(block, &self.account, event.message)
                        .await?
                {
                    let ex = Extrinsic::SignAndSendAuthorities(event.message);
                    self.sender_to_extrinsics
                        .send(ToExtrinsicsMessage::Extrinsic(ex))
                        .await?;
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
                        .map(|s| s.1.clone().0)
                        .collect::<Vec<_>>();
                    let tx_hash = self
                        .ethereum
                        .submit_authorities_set(message, signatures)
                        .await?;
                    info!(
                        target: PangolinRopstenTask::NAME,
                        "Submit authorities to ethereum at {:?} with tx: {}",
                        header.number,
                        tx_hash
                    );
                }
            }
            // call ethereum_backing.lock will emit the event
            EventInfo::ScheduleMMRRootEvent(event) => {
                info!(
                    target: PangolinRopstenTask::NAME,
                    "Find Schedule MMRRoot event at {:?}", header.number
                );
                if self
                    .darwinia2ethereum
                    .is_authority(block, &self.account)
                    .await?
                {
                    info!(target: PangolinRopstenTask::NAME, "{}", event);
                    let ex = Extrinsic::SignAndSendMmrRoot(event.block_number);
                    self.delayed_extrinsics.insert(event.block_number, ex);
                }
            }
            _ => {}
        }
        Ok(())
    }
}
