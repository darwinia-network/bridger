use std::sync::Arc;
use std::time::Duration;

use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use postage::broadcast;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_ethereum::error::BizError;
use component_shadow::{Shadow, ShadowComponent};
use support_ethereum::block::EthereumHeader;

use crate::bus::DarwiniaEthereumBus;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToRelayMessage};
use crate::task::DarwiniaEthereumTask;
use crate::config::SubstrateEthereumConfig;
use bridge_traits::bridge::config::Config;

use async_recursion::async_recursion;
use component_state::state::BridgeState;
use lifeline::dyn_bus::DynBus;

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumRelayService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumRelayService {}

impl Service for LikeDarwiniaWithLikeEthereumRelayService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToRelayMessage>()?;
        let mut sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Config
        let servce_config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-relay", DarwiniaEthereumTask::NAME),
            async move {

                let interval_relay = servce_config.interval_relay;

                tokio::spawn(async move {
                    info!(target: DarwiniaEthereumTask::NAME, "✨ SERVICE STARTED: ETHEREUM <> DARWINIA RELAY");
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
                            trace!(target: DarwiniaEthereumTask::NAME, "Received new ethereum block number to affirm: {}", block_number);
                            update_target(state.clone(), block_number).await
                        },
                        ToRelayMessage::Relay => {
                            affirm(state.clone(), sender_to_extrinsics.clone()).await
                        },
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

#[async_recursion]
async fn update_target(state: BridgeState, block_number: u64) {
    if let Err(err) = LikeDarwiniaWithLikeEthereumRelayService::update_target(state.clone(), block_number).await {
        error!(target: DarwiniaEthereumTask::NAME, "{:#?}", err);
        sleep(Duration::from_secs(30)).await;
        update_target(state, block_number).await;
    }
}

#[async_recursion]
async fn affirm(state: BridgeState, sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>) {
    if let Err(err) = LikeDarwiniaWithLikeEthereumRelayService::affirm(state.clone(), sender_to_extrinsics.clone()).await {
        error!(target: DarwiniaEthereumTask::NAME, "{:#?}", err);
        sleep(Duration::from_secs(30)).await;
        affirm(state, sender_to_extrinsics).await;
    }
}

impl LikeDarwiniaWithLikeEthereumRelayService {
    pub async fn update_target(state: BridgeState, block_number: u64) -> anyhow::Result<()> {
        let microkv = state.microkv();

        let target = microkv.get("target")?.unwrap_or(0);

        if block_number > target {
            microkv.put("target", &block_number)?;
        }

        Ok(())
    }

    pub async fn affirm(state: BridgeState, sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>) -> anyhow::Result<()> {
        let microkv = state.microkv();

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

        // Darwinia client
        let darwinia = component_darwinia.component().await?;
        let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());

        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        let mut relayed= microkv.get("relayed")?.unwrap_or(0);
        let target = microkv.get("target")?.unwrap_or(0);

        trace!(target: DarwiniaEthereumTask::NAME, "Your block to affirm is {}, last confirmed ethereum block is {}", target, last_confirmed);

        if last_confirmed > relayed {
            microkv.put("relayed", &last_confirmed)?;
            relayed = last_confirmed;
        }

        if target > relayed {
            match LikeDarwiniaWithLikeEthereumRelayService::do_affirm(
                ethereum2darwinia,
                shadow,
                target,
                sender_to_extrinsics,
            )
                .await
            {
                Ok(()) => {
                    microkv.put("relayed", &target)?;
                }
                Err(err) => {
                    return Err(err)?;
                }
            }
        }

        Ok(())

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
                if Ethereum2Darwinia::contains(&affirmations, target) {
                    return Err(BizError::AffirmingBlockInGame(target).into());
                }
            }
        }

        trace!(target: DarwiniaEthereumTask::NAME, "Prepare to affirm ethereum block: {}", target);
        let parcel = shadow.parcel(target as usize + 1).await?;
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
}
