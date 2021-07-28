use std::sync::Arc;
use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use postage::broadcast;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_ethereum::error::BizError;
use component_shadow::{Shadow, ShadowComponent};
use component_state::state::BridgeState;
use support_ethereum::block::EthereumHeader;

use crate::bus::PangolinRopstenBus;
use crate::config::SubstrateEthereumConfig;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToRelayMessage};
use crate::task::PangolinRopstenTask;

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumRelayService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumRelayService {}

impl Service for LikeDarwiniaWithLikeEthereumRelayService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToRelayMessage>()?;
        let mut sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Config
        let servce_config: SubstrateEthereumConfig = Config::restore(PangolinRopstenTask::NAME)?;

        // State
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-relay", PangolinRopstenTask::NAME),
            async move {
                let mut helper =
                    RelayHelper::new(state.clone(), sender_to_extrinsics.clone()).await;

                let interval_relay = servce_config.interval_relay;

                tokio::spawn(async move {
                    loop {
                        if let Err(err) = sender_to_relay.send(ToRelayMessage::Relay).await {
                            error!("{:#?}", err);
                        }
                        sleep(Duration::from_secs(interval_relay)).await;
                    }
                });
                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToRelayMessage::EthereumBlockNumber(block_number) => {
                            trace!(
                                target: PangolinRopstenTask::NAME,
                                "Received new ethereum block number to affirm: {}",
                                block_number
                            );
                            helper.update_target(block_number).await?;
                        }
                        ToRelayMessage::Relay => {
                            if let Err(err) = helper.affirm().await {
                                error!(
                                    target: PangolinRopstenTask::NAME,
                                    "affirm err: {:#?}", err
                                );
                                // TODO: Consider the errors more carefully
                                // Maybe a websocket err, so wait 10 secs to reconnect.
                                sleep(Duration::from_secs(10)).await;
                                helper =
                                    RelayHelper::new(state.clone(), sender_to_extrinsics.clone())
                                        .await;
                            }
                        }
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct RelayHelper {
    state: BridgeState,
    sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl RelayHelper {
    #[async_recursion]
    pub async fn new(
        state: BridgeState,
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    ) -> Self {
        match RelayHelper::build(state.clone(), sender_to_extrinsics.clone()).await {
            Ok(helper) => helper,
            Err(err) => {
                error!(
                    target: PangolinRopstenTask::NAME,
                    "relay init err: {:#?}", err
                );
                sleep(Duration::from_secs(10)).await;
                RelayHelper::new(state, sender_to_extrinsics).await
            }
        }
    }

    async fn build(
        state: BridgeState,
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<Self> {
        info!(target: PangolinRopstenTask::NAME, "SERVICE RESTARTING...");

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let component_shadow = ShadowComponent::restore::<PangolinRopstenTask>()?;

        // Darwinia client
        let darwinia = component_darwinia.component().await?;
        let darwinia = Ethereum2Darwinia::new(darwinia.clone());

        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        info!(
            target: PangolinRopstenTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA RELAY"
        );
        Ok(RelayHelper {
            state,
            sender_to_extrinsics,
            darwinia,
            shadow,
        })
    }

    pub async fn affirm(&self) -> anyhow::Result<()> {
        let microkv = self.state.microkv();

        let last_confirmed = self.darwinia.last_confirmed().await?;
        let mut relayed = microkv.get("relayed")?.unwrap_or(0);
        let target = microkv.get("target")?.unwrap_or(0);

        trace!(
            target: PangolinRopstenTask::NAME,
            "The last confirmed ethereum block is {}",
            last_confirmed
        );

        if last_confirmed > relayed {
            microkv.put("relayed", &last_confirmed)?;
            relayed = last_confirmed;
        } else {
            trace!(
            target: PangolinRopstenTask::NAME,
            "The last relayed ethereum block is {}",
            relayed
        );
        }

        if target > relayed {
            trace!(
                target: PangolinRopstenTask::NAME,
                "Your are affirming ethereum block {}",
                target
            );
            if let Err(err) = do_affirm(
                self.darwinia.clone(),
                self.shadow.clone(),
                target,
                self.sender_to_extrinsics.clone(),
            )
            .await {
                    return Err(err);
            }
        } else {
            trace!(
                target: PangolinRopstenTask::NAME,
                "You do not need to affirm ethereum block {}",
                target
            );
        }

        Ok(())
    }

    pub async fn update_target(&self, block_number: u64) -> anyhow::Result<()> {
        let microkv = self.state.microkv();

        let target = microkv.get("target")?.unwrap_or(0);

        if block_number > target {
            microkv.put("target", &block_number)?;
        }

        Ok(())
    }
}

pub async fn do_affirm(
    ethereum2darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
    target: u64,
    mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) -> anyhow::Result<()> {
    // /////////////////////////
    // checking before affirm
    // /////////////////////////
    // 1. pendings check
    let pending_headers = ethereum2darwinia.pending_headers().await?;
    for pending_header in pending_headers {
        let pending_block_number = pending_header.1.header.number;
        if pending_block_number >= target {
            return Err(BizError::AffirmingBlockInPending(target).into());
        }
    }

    // 1. affirmations check
    for (_game_id, game) in ethereum2darwinia.affirmations().await?.iter() {
        for (_round_id, affirmations) in game.iter() {
            if Ethereum2Darwinia::contains(affirmations, target) {
                return Err(BizError::AffirmingBlockInGame(target).into());
            }
        }
    }

    trace!(
        target: PangolinRopstenTask::NAME,
        "Prepare to affirm ethereum block: {}",
        target
    );
    let parcel = shadow.parcel(target as usize).await?;
    if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
        return Err(BizError::ParcelFromShadowIsEmpty(target).into());
    }

    // /////////////////////////
    // do affirm
    // /////////////////////////
    let ex = Extrinsic::Affirm(parcel);
    sender_to_extrinsics
        .send(ToExtrinsicsMessage::Extrinsic(ex))
        .await?;

    Ok(())
}
