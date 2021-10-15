use microkv::namespace::NamespaceMicroKV;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::{
    from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia},
    to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum},
};
use component_ethereum::config::Web3Config;
use component_state::state::BridgeState;
use component_thegraph_liketh::types::{TransactionEntity, TransactionType};
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::receipt::{EthereumReceiptProofThing, RedeemFor};

use crate::message::{EcdsaMessage, Extrinsic};
use crate::task::DarwiniaEthereumTask;

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
                    log::error!(
                        target: DarwiniaEthereumTask::NAME,
                        "extrinsics init err: {:#?}",
                        err
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn build(state: BridgeState) -> anyhow::Result<Self> {
        log::info!(
            target: DarwiniaEthereumTask::NAME,
            "EXTRINSICS SERVICE RESTARTING..."
        );

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;

        // Config
        let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
        let config_web3: Web3Config = Config::restore(DarwiniaEthereumTask::NAME)?;

        // Darwinia client & accounts
        let darwinia = component_darwinia.component().await?;
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

        log::info!(
            target: DarwiniaEthereumTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA EXTRINSICS"
        );

        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
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
    pub async fn send_extrinsic(&self, extrinsic: Extrinsic) -> anyhow::Result<()> {
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

    async fn send_affirm(&self, parcel: EthereumRelayHeaderParcel) -> anyhow::Result<()> {
        let block_number = parcel.header.number;
        let ex_hash = self
            .ethereum2darwinia
            .affirm(&self.ethereum2darwinia_relayer, parcel)
            .await?;
        log::info!(
            target: DarwiniaEthereumTask::NAME,
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
    ) -> anyhow::Result<()> {
        match ethereum_tx.tx_type {
            TransactionType::SetAuthorities => {
                let ex_hash = self
                    .darwinia2ethereum
                    .sync_authorities_change(&self.darwinia2ethereum_relayer, proof)
                    .await?;
                log::info!(
                    target: DarwiniaEthereumTask::NAME,
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
                info!(
                    target: DarwiniaEthereumTask::NAME,
                    "Redeemed ethereum tx {:?} with extrinsic {:?}", ethereum_tx.tx_hash, ex_hash
                );
            }
        }
        Ok(())
    }

    async fn send_guard_vote(&self, pending_block_number: u64, aye: bool) -> anyhow::Result<()> {
        let ex_hash = self
            .ethereum2darwinia
            .vote_pending_relay_header_parcel(
                &self.ethereum2darwinia_relayer,
                pending_block_number,
                aye,
            )
            .await?;
        if aye {
            log::info!(
                target: DarwiniaEthereumTask::NAME,
                "Voted to approve: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        } else {
            log::info!(
                target: DarwiniaEthereumTask::NAME,
                "Voted to reject: {}, ex hash: {:?}",
                pending_block_number,
                ex_hash
            );
        }
        Ok(())
    }

    async fn send_sign_and_send_mmr_root(&self, block_number: u32) -> anyhow::Result<()> {
        log::trace!(
            target: DarwiniaEthereumTask::NAME,
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
        log::info!(
            target: DarwiniaEthereumTask::NAME,
            "Sign and send mmr root of block {} in extrinsic {:?}",
            block_number,
            ex_hash
        );
        Ok(())
    }

    async fn send_sign_and_send_authorities(&self, message: EcdsaMessage) -> anyhow::Result<()> {
        log::trace!(
            target: DarwiniaEthereumTask::NAME,
            "Start sign and send authorities..."
        );
        let ex_hash = self
            .darwinia2ethereum
            .ecdsa_sign_and_submit_signed_authorities(&self.darwinia2ethereum_relayer, message)
            .await?;
        log::info!(
            target: DarwiniaEthereumTask::NAME,
            "Sign and send authorities in extrinsic {:?}",
            ex_hash
        );
        Ok(())
    }
}
