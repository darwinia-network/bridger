use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use component_state::state::BridgeState;

use crate::bus::PangolinRopstenBus;

use crate::message::{ToDarwiniaMessage, ToEthereumMessage, ToRelayMessage, ToRedeemMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use component_ethereum::web3::Web3Component;
use crate::task::PangolinRopstenTask;
use bridge_traits::bridge::component::BridgeComponent;
use web3::types::{TransactionId, H256};
use array_bytes::hex2bytes_unchecked as bytes;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;

use bridge_traits::bridge::task::BridgeSand;

pub async fn dispatch_route(
    bus: &PangolinRopstenBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay(bus, param).await,
        "redeem" => redeem(bus, param).await,
        "start-pangolin" => start_pangolin(bus, param).await,
        "start-ropsten" => start_ropsten(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}

async fn relay(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToRelayMessage>()?;
    let block_number = param
        .get("block_number")
        .ok_or_else(|| StandardError::Api("The block_number is required".to_string()))?;
    let block_number = block_number.as_str().unwrap();
    sender
        .send(ToRelayMessage::EthereumBlockNumber(
            block_number.parse::<u64>().unwrap(),
        ))
        .await?;
    // todo: there can be upgrade config to set `auto_start=true`
    Ok(TaskTerminal::new("success"))
}

async fn redeem(
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

    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
    let darwinia = component_darwinia_subxt.component().await?;

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
        index: tx.transaction_index.unwrap().as_u64()
    };


    if darwinia
        .verified(eth_tx.block_hash, eth_tx.index)
        .await?
    {
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
    }

    ;

    Ok(TaskTerminal::new("success"))
}

async fn start_pangolin(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToDarwiniaMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u32>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-tracked-pangolin-block", &block_number)?;
    }

    sender.send(ToDarwiniaMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}

async fn start_ropsten(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToEthereumMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u64>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-redeemed-ropsten", &block_number)?;
    }

    sender.send(ToEthereumMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}
