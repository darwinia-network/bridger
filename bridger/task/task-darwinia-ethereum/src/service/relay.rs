use std::sync::Arc;
use std::time::{Duration, SystemTime};

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
        let mut sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

        // Config
        let servce_config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

        let _greet = Self::try_task(
            &format!("{}-service-relay", DarwiniaEthereumTask::NAME),
            async move {
                info!(target: DarwiniaEthereumTask::NAME, "✨ SERVICE STARTED: ETHEREUM > DARWINIA RELAY");

                // Darwinia client
                let darwinia = component_darwinia.component().await?;
                let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());

                // Shadow client
                let shadow = Arc::new(component_shadow.component().await?);

                let interval_relay = servce_config.interval_relay;

                let last_confirmed = ethereum2darwinia.last_confirmed().await?;
                let mut target: u64 = last_confirmed;
                let mut relayed: u64 = last_confirmed;

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
                            if block_number > target {
                                target = block_number;
                            }
                        },
                        ToRelayMessage::Relay => {
                            if target > relayed {
                                match LikeDarwiniaWithLikeEthereumRelayService::affirm(
                                    ethereum2darwinia.clone(),
                                    shadow.clone(),
                                    target,
                                    sender_to_extrinsics.clone(),
                                    )
                                    .await
                                    {
                                        Ok(()) => {
                                            relayed = target;
                                        }
                                        Err(err) => {
                                            error!("{:#?}", err);
                                        }
                                    }
                            }

                        },
                        _ => {}
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

impl LikeDarwiniaWithLikeEthereumRelayService {
    pub async fn affirm(
        ethereum2darwinia: Ethereum2Darwinia,
        shadow: Arc<Shadow>,
        target: u64,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<()> {
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. last confirmed check
        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        if target <= last_confirmed {
            return Err(
                BizError::AffirmingBlockLessThanLastConfirmed(target, last_confirmed).into(),
            );
        }

        // 2. pendings check
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                return Err(BizError::AffirmingBlockInPending(target).into());
            }
        }

        // 3. affirmations check
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
