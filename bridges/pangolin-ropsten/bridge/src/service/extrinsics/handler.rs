use std::time::{SystemTime, UNIX_EPOCH};

use microkv::namespace::NamespaceMicroKV;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::config::ClientConfig;
use client_pangolin::types::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use client_pangolin::types::runtime_types::to_ethereum_backing::pallet::RedeemFor;
use client_pangolin::types::{EcdsaMessage, EthereumAccount, EthereumReceiptProofThing};
use component_ethereum::web3::Web3Config;
use component_state::state::BridgeState;
use component_thegraph_liketh::types::{TransactionEntity, TransactionType};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;

use crate::bridge::Extrinsic;
use crate::bridge::{PangolinRopstenConfig, PangolinRopstenTask};

pub struct ExtrinsicsHandler {
    client: PangolinClient,
    ethereum_account: EthereumAccount,
    microkv: NamespaceMicroKV,
    message_kv: NamespaceMicroKV,
}

impl ExtrinsicsHandler {
    pub async fn new(state: BridgeState) -> Self {
        loop {
            match Self::build(state.clone()).await {
                Ok(handler) => return handler,
                Err(err) => {
                    tracing::error!(
                        target: "pangolin-ropsten",
                        "[pangolin] [extrinsics] extrinsics init err: {:#?}",
                        err
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn build(state: BridgeState) -> color_eyre::Result<Self> {
        tracing::info!(
            target: "pangolin-ropsten",
            "EXTRINSICS SERVICE RESTARTING..."
        );
        let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

        // Config
        let config_darwinia: ClientConfig = bridge_config.darwinia;
        let config_web3: Web3Config = bridge_config.web3;

        let ethereum_account = EthereumAccount::new(
            config_web3.endpoint,
            config_darwinia.ecdsa_authority_private_key.clone(),
        );

        let client = PangolinClientComponent::component(config_darwinia).await?;

        tracing::info!(
            target: "pangolin-ropsten",
            "✨ SERVICE STARTED: ROPSTEN <> PANGOLIN EXTRINSICS"
        );

        let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());
        let message_kv: NamespaceMicroKV =
            state.microkv_with_namespace(format!("{}-messages", PangolinRopstenTask::name()));
        Ok(ExtrinsicsHandler {
            client,
            ethereum_account,
            microkv,
            message_kv,
        })
    }
}

impl ExtrinsicsHandler {
    pub async fn send_extrinsic(&self, extrinsic: Extrinsic) -> color_eyre::Result<()> {
        match extrinsic {
            Extrinsic::Affirm(parcel) => self.send_affirm(parcel).await?,
            Extrinsic::Redeem(proof, ethereum_tx) => self.send_redeem(proof, ethereum_tx).await?,
            Extrinsic::GuardVote(pending_block_number, aye) => {
                self.send_guard_vote(pending_block_number, aye).await?
            }
            Extrinsic::SignAndSendMmrRoot(block_number) => {
                self.send_sign_and_send_mmr_root(block_number).await?
            }
            Extrinsic::SignAndSendAuthorities(message) => {
                self.send_sign_and_send_authorities(message).await?
            }
        }
        // Delay for waiting to fininsh
        tokio::time::sleep(std::time::Duration::from_secs(12)).await;
        Ok(())
    }

    async fn send_affirm(&self, parcel: EthereumRelayHeaderParcel) -> color_eyre::Result<()> {
        let block_number = parcel.header.number;
        let ex_hash = self.client.ethereum().affirm(parcel).await?;
        tracing::info!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Affirmed ethereum block {} in extrinsic {:?}",
            block_number,
            ex_hash
        );
        self.microkv.put("relayed", &block_number)?;
        Ok(())
    }

    async fn send_redeem(
        &self,
        proof: EthereumReceiptProofThing,
        ethereum_tx: TransactionEntity,
    ) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Ready to send redeem. type is [{:?}] and tx is [{:?}]",
            ethereum_tx.tx_type,
            ethereum_tx.tx_hash
        );
        match ethereum_tx.tx_type {
            TransactionType::SetAuthorities => {
                let ex_hash = self
                    .client
                    .ethereum()
                    .sync_authorities_change(proof)
                    .await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "[pangolin] [extrinsics] Sent ethereum tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            TransactionType::RegisterErc20Token => {
                let ex_hash = self.client.ethereum().register_erc20(proof).await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "[pangolin] [extrinsics] register erc20 token tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            TransactionType::RedeemErc20Token => {
                let ex_hash = self.client.ethereum().redeem_erc20(proof).await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "[pangolin] [extrinsics] redeem erc20 token tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            _ => {
                let redeem_for = match ethereum_tx.tx_type {
                    TransactionType::Deposit => RedeemFor::Deposit,
                    TransactionType::Token => RedeemFor::Token,
                    // TransactionType::SetAuthorities => RedeemFor::SetAuthorities,
                    // TransactionType::RegisterErc20Token => RedeemFor::RegisterErc20Token,
                    // TransactionType::RedeemErc20Token => RedeemFor::RedeemErc20Token,
                    _ => return Err(BridgerError::Custom("Unreachable".to_string()).into()),
                };
                let ex_hash = self.client.ethereum().redeem(redeem_for, proof).await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "[pangolin] [extrinsics] Redeemed ethereum tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
        }
        Ok(())
    }

    async fn send_guard_vote(
        &self,
        pending_block_number: u64,
        aye: bool,
    ) -> color_eyre::Result<()> {
        let ex_hash = self
            .client
            .runtime()
            .tx()
            .ethereum_relay()
            .vote_pending_relay_header_parcel(pending_block_number, aye)
            .sign_and_submit(self.client.account().signer())
            .await?;
        if aye {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [extrinsics] Voted to approve: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        } else {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [extrinsics] Voted to reject: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        }
        Ok(())
    }

    async fn send_sign_and_send_mmr_root(&self, block_number: u32) -> color_eyre::Result<()> {
        tracing::trace!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Start sign and send mmr_root for block: {}",
            block_number,
        );

        let ex_hash = self
            .client
            .ethereum()
            .ecdsa_sign_and_submit_signed_mmr_root(self.ethereum_account.clone(), block_number)
            .await?;
        tracing::info!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Sign and send mmr root of block {} in extrinsic {:?}",
            block_number,
            ex_hash
        );
        Ok(())
    }

    async fn send_sign_and_send_authorities(
        &self,
        message: EcdsaMessage,
    ) -> color_eyre::Result<()> {
        tracing::trace!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Start sign and send authorities..."
        );
        let ex_hash = self
            .client
            .ethereum()
            .ecdsa_sign_and_submit_signed_authorities(self.ethereum_account.clone(), message)
            .await?;
        tracing::info!(
            target: "pangolin-ropsten",
            "[pangolin] [extrinsics] Sign and send authorities in extrinsic {:?}",
            ex_hash
        );
        Ok(())
    }
}

impl ExtrinsicsHandler {
    pub fn collect_message(&self, message: &Extrinsic) -> color_eyre::Result<()> {
        let mut key: String = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string();

        if let Ok(true) = self.message_kv.exists(&key) {
            let random: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(1)
                .map(char::from)
                .collect();
            key += &random;
        }
        self.message_kv.put(key, message)?;
        Ok(())
    }

    pub async fn consume_message(&self) -> color_eyre::Result<()> {
        loop {
            let extrinsics = self.message_kv.sorted_keys()?;
            if extrinsics.is_empty() {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                continue;
            }
            for key in extrinsics.iter() {
                let ex: Extrinsic = self.message_kv.get_as_unwrap(&key)?;
                match self.send_extrinsic(ex.clone()).await {
                    Ok(_) => self.message_kv.delete(&key)?,
                    Err(err) => {
                        self.message_kv.delete(&key)?;
                        tracing::error!(
                            target: "pangolin-ropsten",
                            "[pangolin] [extrinsics] Failed to send extrinsic {:?} err: {:?}",
                            ex,
                            err
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    }
                }
            }
        }
    }
}
