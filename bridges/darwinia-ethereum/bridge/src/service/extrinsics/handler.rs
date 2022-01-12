use microkv::namespace::NamespaceMicroKV;

use client_darwinia::account::DarwiniaAccount;
use client_darwinia::component::DarwiniaSubxtComponent;
use client_darwinia::config::DarwiniaSubxtConfig;
use client_darwinia::{
    from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia},
    to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum},
};
use component_ethereum::web3::Web3Config;
use component_state::state::BridgeState;
use component_thegraph_liketh::types::{TransactionEntity, TransactionType};
use support_common::config::{Config, Names};
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::receipt::{EthereumReceiptProofThing, RedeemFor};

use crate::bridge::{DarwiniaEthereumConfig, DarwiniaEthereumTask};
use crate::bridge::{EcdsaMessage, Extrinsic};

pub struct ExtrinsicsHandler {
    ethereum2darwinia: Ethereum2Darwinia,
    darwinia2ethereum: Darwinia2Ethereum,
    darwinia2ethereum_relayer: ToEthereumAccount,
    ethereum2darwinia_relayer: FromEthereumAccount,
    spec_name: String,
    microkv: NamespaceMicroKV,
}

impl ExtrinsicsHandler {
    pub async fn new(state: BridgeState) -> Self {
        loop {
            match Self::build(state.clone()).await {
                Ok(handler) => return handler,
                Err(err) => {
                    tracing::error!(
                        target: "darwinia-ethereum",
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
            target: "darwinia-ethereum",
            "EXTRINSICS SERVICE RESTARTING..."
        );
        let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

        // Config
        let config_darwinia: DarwiniaSubxtConfig = bridge_config.darwinia;
        let config_web3: Web3Config = bridge_config.web3;

        // Darwinia client & accounts
        let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
        let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
        let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
        let account = DarwiniaAccount::new(
            config_darwinia.relayer_private_key,
            config_darwinia.relayer_real_account,
        );
        let darwinia2ethereum_relayer = ToEthereumAccount::new(
            account.clone(),
            config_darwinia.ecdsa_authority_private_key,
            config_web3.endpoint,
        );
        let ethereum2darwinia_relayer = FromEthereumAccount::new(account);

        let spec_name = darwinia.runtime_version().await?;

        tracing::info!(
            target: "darwinia-ethereum",
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA EXTRINSICS"
        );

        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::name());
        Ok(ExtrinsicsHandler {
            ethereum2darwinia,
            darwinia2ethereum,
            darwinia2ethereum_relayer,
            ethereum2darwinia_relayer,
            spec_name,
            microkv,
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
        let ex_hash = self
            .ethereum2darwinia
            .affirm(&self.ethereum2darwinia_relayer, parcel)
            .await?;
        tracing::info!(
            target: "darwinia-ethereum",
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
        match ethereum_tx.tx_type {
            TransactionType::SetAuthorities => {
                let ex_hash = self
                    .darwinia2ethereum
                    .sync_authorities_change(&self.darwinia2ethereum_relayer, proof)
                    .await?;
                tracing::info!(
                    target: "darwinia-ethereum",
                    "Sent ethereum tx {:?} with extrinsic {:?}",
                    ethereum_tx.tx_hash,
                    ex_hash
                );
            }
            TransactionType::RegisterErc20Token => {}
            TransactionType::RedeemErc20Token => {}
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
                    target: "darwinia-ethereum",
                    "Redeemed ethereum tx {:?} with extrinsic {:?}", ethereum_tx.tx_hash, ex_hash
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
                target: "darwinia-ethereum",
                "Voted to approve: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        } else {
            tracing::info!(
                target: "darwinia-ethereum",
                "Voted to reject: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        }
        Ok(())
    }

    async fn send_sign_and_send_mmr_root(&self, block_number: u32) -> color_eyre::Result<()> {
        tracing::trace!(
            target: "darwinia-ethereum",
            "Start sign and send mmr_root..."
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
            target: "darwinia-ethereum",
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
            target: "darwinia-ethereum",
            "Start sign and send authorities..."
        );
        let ex_hash = self
            .darwinia2ethereum
            .ecdsa_sign_and_submit_signed_authorities(&self.darwinia2ethereum_relayer, message)
            .await?;
        tracing::info!(
            target: "darwinia-ethereum",
            "Sign and send authorities in extrinsic {:?}",
            ex_hash
        );
        Ok(())
    }
}