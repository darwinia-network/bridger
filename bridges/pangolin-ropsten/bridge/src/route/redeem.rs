use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::{Bus, Sender};
use web3::types::{TransactionId, H256};

use client_pangolin::component::DarwiniaSubxtComponent;
use component_ethereum::web3::Web3Component;
use component_thegraph_liketh::types::{TransactionEntity, TransactionType};

use crate::bridge::PangolinRopstenBus;
use crate::bridge::PangolinRopstenTask;
use crate::bridge::ToRedeemMessage;

pub async fn redeem(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> color_eyre::Result<TaskTerminal> {
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

    let tx_type = match the_type {
        "token" => TransactionType::Token,
        "deposit" => TransactionType::Deposit,
        "set_authorities" => TransactionType::SetAuthorities,
        _ => color_eyre::bail!("err"),
    };
    let tx_hash = array_bytes::bytes2hex("0x", tx.hash);
    let block_hash_h256 = tx.block_hash.unwrap();
    let eth_tx = TransactionEntity {
        id: tx_hash.clone(),
        origin: tx_type.belong(),
        block_number: tx.block_number.unwrap().as_u64(),
        block_hash: array_bytes::bytes2hex("0x", block_hash_h256),
        tx_hash,
        tx_index: tx.transaction_index.unwrap().as_u64(),
        tx_type,
    };

    if darwinia
        .verified(
            substrate_subxt::sp_core::H256(block_hash_h256.to_fixed_bytes()),
            eth_tx.tx_index,
        )
        .await?
    {
        trace!(
            target: "pangolin-ropsten",
            "This ethereum tx {:?} has already been redeemed.",
            &eth_tx.tx_hash
        );
    } else {
        trace!(
            target: "pangolin-ropsten",
            "send to redeem service: {:?}",
            &eth_tx.tx_hash
        );
        sender
            .send(ToRedeemMessage::EthereumTransaction(eth_tx))
            .await?
    };

    Ok(TaskTerminal::new("success"))
}
