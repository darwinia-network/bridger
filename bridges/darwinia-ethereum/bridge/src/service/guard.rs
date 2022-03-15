use std::time::Duration;

use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use tokio::time::sleep;

use client_darwinia::client::DarwiniaClient;
use client_darwinia::component::DarwiniaClientComponent;
use client_darwinia::types::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use component_ethereum::errors::BizError;
use component_shadow::component::ShadowComponent;
use component_shadow::shadow::Shadow;
use component_state::state::BridgeState;
use lifeline::dyn_bus::DynBus;
use microkv::namespace::NamespaceMicroKV;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::DarwiniaEthereumBus;
use crate::bridge::DarwiniaEthereumTask;
use crate::bridge::{DarwiniaEthereumConfig, Extrinsic, ToExtrinsicsMessage, ToGuardMessage};

#[derive(Debug)]
pub struct GuardService {
    _greet: Lifeline,
}

impl BridgeService for GuardService {}

impl Service for GuardService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToGuardMessage>()?;
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::name());

        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", DarwiniaEthereumTask::name()),
            async move {
                //
                tokio::spawn(async move { start(sender_to_extrinsics, &microkv).await });

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToGuardMessage::StartGuard => {}
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(
    mut sender_to_extrinsics: impl Sender<ToExtrinsicsMessage>,
    microkv: &NamespaceMicroKV,
) {
    while let Err(err) = run(&mut sender_to_extrinsics, microkv).await {
        tracing::error!(target: "darwinia-ethereum", "[darwinia] [guard] guard err {:#?}", err);
        sleep(Duration::from_secs(10)).await;
    }
}

async fn run(
    sender_to_extrinsics: &mut impl Sender<ToExtrinsicsMessage>,
    microkv: &NamespaceMicroKV,
) -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-ethereum", "SERVICE RESTARTING...");

    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

    // Config
    let config_darwinia = bridge_config.darwinia;
    let servce_config = bridge_config.task;

    // Darwinia client & account
    let client = DarwiniaClientComponent::component(config_darwinia).await?;
    let is_tech_comm_member = client.is_tech_comm_member(None, None).await?;
    if !is_tech_comm_member {
        return Ok(());
    }

    // Shadow client
    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
    )?;

    tracing::info!(
        target: "darwinia-ethereum",
        "✨ SERVICE STARTED: ETHEREUM <> DARWINIA GUARD"
    );

    loop {
        GuardService::guard(&client, &shadow, sender_to_extrinsics, microkv).await?;
        sleep(Duration::from_secs(servce_config.interval_guard)).await;
    }
}

impl GuardService {
    pub async fn extrinsics(
        client: &DarwiniaClient,
        shadow: &Shadow,
    ) -> color_eyre::Result<Vec<Extrinsic>> {
        tracing::trace!(
            target: "darwinia-ethereum",
            "[darwinia] [guard] Checking pending headers..."
        );

        let mut extrinsics = Vec::new();

        let last_confirmed = client.ethereum().last_confirmed().await?;
        let pending_headers = client
            .runtime()
            .storage()
            .ethereum_relay()
            .pending_relay_header_parcels(None)
            .await?;

        if !pending_headers.is_empty() {
            tracing::trace!(
                target: "darwinia-ethereum",
                "[darwinia] [guard] pending headers: {:?}",
                pending_headers
                    .clone()
                    .iter()
                    .map(|p| p.1.header.number.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        for pending in pending_headers {
            let pending_parcel = pending.1;
            let voting_state = pending.2;
            let pending_block_number: u64 = pending_parcel.header.number;

            // high than last_confirmed(https://github.com/darwinia-network/bridger/issues/33),
            // and,
            // have not voted
            if last_confirmed <= pending_block_number {
                continue;
            }
            let real_account = client.account().real_account();
            let has_voted = voting_state.ayes.contains(real_account)
                || voting_state.nays.contains(real_account);
            if has_voted {
                continue;
            }

            match shadow.parcel(pending_block_number as usize).await {
                Ok(parcel_from_shadow) => {
                    let parcel_from_shadow: EthereumRelayHeaderParcel =
                        parcel_from_shadow.try_into()?;
                    let ex = if is_same_as(&pending_parcel, &parcel_from_shadow) {
                        Extrinsic::GuardVote(pending_block_number, true)
                    } else {
                        Extrinsic::GuardVote(pending_block_number, false)
                    };
                    extrinsics.push(ex);
                }
                Err(err) => {
                    if let Some(BizError::BlankEthereumMmrRoot(block, msg)) =
                        err.downcast_ref::<BizError>()
                    {
                        tracing::warn!(
                            target: "darwinia-ethereum",
                            "[darwinia] [guard] The parcel of ethereum block {} from Shadow service is blank, the err msg is {}",
                            block,
                            msg
                        );
                        return Ok(extrinsics);
                    }
                    return Err(err);
                }
            }
        }

        Ok(extrinsics)
    }

    async fn guard(
        client: &DarwiniaClient,
        shadow: &Shadow,
        sender_to_extrinsics: &mut impl Sender<ToExtrinsicsMessage>,
        microkv: &NamespaceMicroKV,
    ) -> color_eyre::Result<()> {
        let extrinsics = Self::extrinsics(client, shadow).await?;

        if extrinsics.is_empty() {
            return Ok(());
        }

        let max_block = *extrinsics
            .iter()
            .map(|ex| {
                if let Extrinsic::GuardVote(block_num, _) = ex {
                    block_num
                } else {
                    &0u64
                }
            })
            .max()
            .unwrap();

        let latest: u64 = microkv
            .get_as_unwrap("latest_guard_vote_block_num")
            .unwrap_or(0u64);
        for extrinsic in extrinsics {
            if let Extrinsic::GuardVote(block_num, _) = extrinsic {
                if block_num > latest {
                    let message = ToExtrinsicsMessage::Extrinsic(extrinsic);
                    sender_to_extrinsics.send(message).await?;
                } else {
                    tracing::info!(
                        target: "darwinia-ethereum",
                        "[darwinia] [guard] Skip guard vote for block: {}",
                        &block_num
                    );
                }
            }
        }
        if max_block > latest {
            microkv.put("latest_guard_vote_block_num", &max_block)?;
        }

        Ok(())
    }
}

fn is_same_as(a: &EthereumRelayHeaderParcel, b: &EthereumRelayHeaderParcel) -> bool {
    a.header.hash == b.header.hash && a.parent_mmr_root == b.parent_mmr_root
}
