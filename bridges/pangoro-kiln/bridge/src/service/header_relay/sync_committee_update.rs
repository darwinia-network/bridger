use std::ops::Div;

use crate::{
    bridge::{BridgeConfig, PangoroKilnBus},
    kiln_client::{client::KilnClient, types::Proof},
    pangoro_client::client::PangoroClient,
};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{
    contract::{tokens::Tokenize, Options},
    ethabi::Token,
    types::U256,
};

#[derive(Debug)]
pub struct SyncCommitteeUpdateService {
    _greet: Lifeline,
}

impl BridgeService for SyncCommitteeUpdateService {}

impl Service for SyncCommitteeUpdateService {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("sync-committee-update-kiln-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro sync committee update relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    println!("{:?}", &config);
    let pangoro_client = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.contract_abi_path,
        &config.pangoro.contract_address,
        &config.pangoro.execution_layer_contract_abi_path,
        &config.pangoro.execution_layer_contract_address,
        &config.pangoro.private_key,
    )?;
    let kiln_client = KilnClient::new(&config.kiln.endpoint)?;
    let update_manager = SyncCommitteeUpdate {
        pangoro_client,
        kiln_client,
    };

    loop {
        if let Err(error) = update_manager.sync_committee_update().await {
            tracing::error!(
                target: "pangoro-kiln",
                "[SyncCommittee][Kiln => Pangoro] Failed relay sync committee update : {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

pub struct SyncCommitteeUpdate {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl SyncCommitteeUpdate {
    pub async fn sync_committee_update(&self) -> color_eyre::Result<()> {
        let last_relayed_header = self.pangoro_client.finalized_header().await?;
        let period = last_relayed_header.slot.div(32).div(256);

        let _current_sync_committee = self.pangoro_client.sync_committee_roots(period).await?;
        let next_sync_committee = self.pangoro_client.sync_committee_roots(period + 1).await?;
        if next_sync_committee.is_zero() {
            tracing::info!(
                target: "pangoro-kiln",
                "[SyncCommittee][Kiln => Pangoro] Try to relay SyncCommittee at period {:?}",
                period + 1,
            );

            let parameter = self
                .get_sync_committee_update_parameter(period, last_relayed_header.slot)
                .await?;
            let tx = self
                .pangoro_client
                .contract
                .signed_call(
                    "import_next_sync_committee",
                    (parameter,),
                    Options {
                        gas: Some(U256::from(10000000)),
                        gas_price: Some(U256::from(1300000000)),
                        ..Default::default()
                    },
                    &self.pangoro_client.private_key,
                )
                .await?;

            tracing::info!(
                target: "pangoro-kiln",
                "[SyncCommittee][Kiln => Pangoro] Sending tx: {:?}",
                &tx
            );
        } else {
            tracing::info!(
                target: "pangoro-kiln",
                "[SyncCommittee][Kiln => Pangoro] Next sync committee is {:?}",
                next_sync_committee
            );
        }
        Ok(())
    }

    async fn get_sync_committee_update_parameter(
        &self,
        period: u64,
        slot: u64,
    ) -> color_eyre::Result<Token> {
        let sync_committee_update = self
            .kiln_client
            .get_sync_committee_period_update(period, 1)
            .await?;
        if sync_committee_update.is_empty() {
            return Err(BridgerError::Custom("Failed to get sync committee update".into()).into());
        }
        let next_sync_committee = sync_committee_update
            .get(0)
            .expect("Unreachable!")
            .next_sync_committee
            .clone();
        let next_sync_committee_branch = self
            .kiln_client
            .get_next_sync_committee_branch(slot)
            .await?;
        let witnesses = match next_sync_committee_branch {
            Proof::SingleProof {
                gindex: _,
                leaf: _,
                witnesses,
            } => witnesses,
            _ => return Err(BridgerError::Custom("Not implemented!".to_string()).into()),
        };

        let next_sync_committee = Token::Tuple(
            (
                Token::FixedArray(
                    next_sync_committee
                        .pubkeys
                        .iter()
                        .map(|s| hex::decode(&s.clone()[2..]))
                        .collect::<Result<Vec<Vec<u8>>, _>>()?
                        .iter()
                        .map(|s| Token::Bytes(s.to_vec()))
                        .collect::<Vec<Token>>(),
                ),
                hex::decode(&next_sync_committee.aggregate_pubkey.clone()[2..])?,
            )
                .into_tokens(),
        );
        Ok(Token::Tuple((next_sync_committee, witnesses).into_tokens()))
    }
}
