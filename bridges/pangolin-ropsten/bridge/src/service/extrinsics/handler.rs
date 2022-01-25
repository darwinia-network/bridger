use std::time::{SystemTime, UNIX_EPOCH};

use microkv::namespace::NamespaceMicroKV;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::config::ClientConfig;
use client_pangolin::types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use client_pangolin::types::{DarwiniaAccount, EthereumReceiptProofThing};
use component_ethereum::web3::Web3Config;
use component_state::state::BridgeState;
use component_thegraph_liketh::types::{TransactionEntity, TransactionType};
use support_common::config::{Config, Names};

use crate::bridge::{EcdsaMessage, Extrinsic};
use crate::bridge::{PangolinRopstenConfig, PangolinRopstenTask};

pub struct ExtrinsicsHandler {
    // ethereum2darwinia: Ethereum2Darwinia,
    // darwinia2ethereum: Darwinia2Ethereum,
    // darwinia2ethereum_relayer: ToEthereumAccount,
    // ethereum2darwinia_relayer: FromEthereumAccount,
    // darwinia_account: DarwiniaAccount,
    client: PangolinClient,
    spec_name: String,
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
                        "extrinsics init err: {:#?}",
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
        // let config_web3: Web3Config = bridge_config.web3;
        //
        // Darwinia client & accounts
        // let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
        // let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
        // let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
        // let account = DarwiniaAccount::new(
        //     config_darwinia.relayer_private_key,
        //     config_darwinia.relayer_real_account,
        // );
        // let darwinia2ethereum_relayer = ToEthereumAccount::new(
        //     account.clone(),
        //     config_darwinia.ecdsa_authority_private_key,
        //     config_web3.endpoint,
        // );
        // let ethereum2darwinia_relayer = FromEthereumAccount::new(account);
        //
        // let spec_name = darwinia.runtime_version().await?;

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
            spec_name,
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
            "Affirmed ethereum block {} in extrinsic {:?}",
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
            "Ready to send redeem. type is [{:?}] and tx is [{:?}]",
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
                    "Sent ethereum tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            TransactionType::RegisterErc20Token => {
                let ex_hash = self.client.ethereum().register_erc20(proof).await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "register erc20 token tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            TransactionType::RedeemErc20Token => {
                let ex_hash = self
                    .ethereum2darwinia
                    .redeem_erc20(&self.ethereum2darwinia_relayer, proof)
                    .await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "redeem erc20 token tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            _ => {
                let redeem_for = match ethereum_tx.tx_type {
                    TransactionType::Deposit => RedeemFor::Deposit,
                    TransactionType::Token => RedeemFor::Token,
                    TransactionType::SetAuthorities => RedeemFor::SetAuthorities,
                    TransactionType::RegisterErc20Token => RedeemFor::RegisterErc20Token,
                    TransactionType::RedeemErc20Token => RedeemFor::RedeemErc20Token,
                };
                let ex_hash = self
                    .ethereum2darwinia
                    .redeem(&self.ethereum2darwinia_relayer, redeem_for, proof)
                    .await?;
                tracing::info!(
                    target: "pangolin-ropsten",
                    "Redeemed ethereum tx {:?} with extrinsic {:?}",
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
            .ethereum2darwinia
            .vote_pending_relay_header_parcel(
                &self.ethereum2darwinia_relayer,
                pending_block_number,
                aye,
            )
            .await?;
        if aye {
            tracing::info!(
                target: "pangolin-ropsten",
                "Voted to approve: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        } else {
            tracing::info!(
                target: "pangolin-ropsten",
                "Voted to reject: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        }
        Ok(())
    }

    async fn send_sign_and_send_mmr_root(&self, block_number: u32) -> color_eyre::Result<()> {
        tracing::trace!(
            target: "pangolin-ropsten",
            "Start sign and send mmr_root for block: {}",
            block_number,
        );
        let ex_hash = self
            .darwinia2ethereum
            .ecdsa_sign_and_submit_signed_mmr_root(
                &self.darwinia2ethereum_relayer,
                self.spec_name.clone(),
                block_number,
            )
            .await?;
        tracing::info!(
            target: "pangolin-ropsten",
            "Sign and send mmr root of block {} in extrinsic {:?}",
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
            "Start sign and send authorities..."
        );
        let ex_hash = self
            .darwinia2ethereum
            .ecdsa_sign_and_submit_signed_authorities(&self.darwinia2ethereum_relayer, message)
            .await?;
        tracing::info!(
            target: "pangolin-ropsten",
            "Sign and send authorities in extrinsic {:?}",
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
                        if let Some(substrate_subxt::Error::Rpc(_)) =
                            err.downcast_ref::<substrate_subxt::Error>()
                        {
                            tracing::warn!(
                                target: "pangolin-ropsten",
                                "Connection Error. Try to resend later. extrinsic: {:?}",
                                ex,
                            );
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            break;
                        } else {
                            self.message_kv.delete(&key)?;
                            tracing::error!(
                                target: "pangolin-ropsten",
                                "Failed to send extrinsic {:?} err: {:?}",
                                ex,
                                err
                            );
                        }
                    }
                }
            }
        }
    }
}
