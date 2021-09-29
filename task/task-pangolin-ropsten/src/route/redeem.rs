use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::{Bus, Sender};
use web3::types::{TransactionId, H256};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use component_ethereum::web3::Web3Component;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use support_ethereum::transaction::{EthereumTransaction, EthereumTransactionHash};

use crate::bus::PangolinRopstenBus;
use crate::message::ToRedeemMessage;
use crate::task::PangolinRopstenTask;

pub async fn redeem(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToRedeemMessage>()?;
    // param: type, cross-chain transaction type
    let the_type = param
        .get("type")
        .ok_or_else(|| StandardError::Api("The type is required".to_string()))?;
    let the_type = the_type.as_str().unwrap();

    // param: eth_txhash
    let eth_txhash = param
        .get("eth_txhash")
        .ok_or_else(|| StandardError::Api("The eth_txhash is required".to_string()))?;
    let eth_txhash = eth_txhash.as_str().unwrap();

    let component_web3 = Web3Component::restore::<PangolinRopstenTask>()?;
    let web3 = component_web3.component().await?;
    let tx_id = TransactionId::Hash(H256::from_slice(&bytes(eth_txhash)));
    let tx = web3.eth().transaction(tx_id).await?.unwrap();

    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
    let darwinia = component_pangolin_subxt.component().await?;

    let tx_hash = if the_type == "token" {
        EthereumTransactionHash::Token(tx.hash)
    } else if the_type == "deposit" {
        EthereumTransactionHash::Deposit(tx.hash)
    } else if the_type == "set_authorities" {
        EthereumTransactionHash::SetAuthorities(tx.hash)
    } else {
        anyhow::bail!("err")
    };
    let eth_tx = EthereumTransaction {
        tx_hash,
        block_hash: tx.block_hash.unwrap(),
        block: tx.block_number.unwrap().as_u64(),
        index: tx.transaction_index.unwrap().as_u64(),
    };

    if darwinia.verified(eth_tx.block_hash, eth_tx.index).await? {
        trace!(
            target: PangolinRopstenTask::NAME,
            "This ethereum tx {:?} has already been redeemed.",
            eth_tx.enclosed_hash()
        );
    } else {
        trace!(
            target: PangolinRopstenTask::NAME,
            "send to redeem service: {:?}",
            &eth_tx.tx_hash
        );
        sender
            .send(ToRedeemMessage::EthereumTransaction(eth_tx))
            .await?
    };

    Ok(TaskTerminal::new("success"))
}
