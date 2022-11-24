use std::{str::FromStr, time::Duration};

use bridge_e2e_traits::client::{MessageClient, Web3Client};
use bridge_pangoro_goerli::service::message_relay::pangoro_to_goerli::message_relay_client_builder;
use client_contracts::outbound_types::SendMessage;
use relay_e2e::types::ethereum::FastEthereumAccount;
use support_etherscan::wait_for_transaction_confirmation;
use web3::{contract::Options, ethabi::Address, types::U256};

#[test]
fn test_signing() {
    let message = array_bytes::hex2bytes(
        "0x331a5c39bad492d36b8306eb45792c3198c374eb0dc188bc704729f9330093f3",
    )
    .unwrap();
    let seed = "0x40b50cd43ccbfe7da7e594216710eac2ab0036fa59a957a85c5d8ee4f3761f49";
    let eth_account = FastEthereumAccount::new(seed);
    let signature = eth_account.sign(message.as_slice()).unwrap();
    let expected = "0x9d534608bb6a55ebf900e4835e90d0355aa4e30830ba3e3f6f3fdf913b59fec138412bc5957975f23370ea2a035e9b4d6a69a9effc4a32de0789490b4a0947d701";
    let compare = array_bytes::bytes2hex("0x", &signature);
    assert_eq!(&compare[..], expected);
}

#[tokio::test]
async fn test_msg_darwinia_to_eth() -> color_eyre::Result<()> {
    let msg = message_relay_client_builder().await?;
    // Get fee from fee market
    let relayer_info = msg.source.strategy.fee_market.get_relayer_info().await?;
    dbg!(&relayer_info);
    let fee = relayer_info.get(0).expect("There are no relayers!").fee;
    dbg!(fee);
    // Send messages
    let message = SendMessage {
        target_contract: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        encoded: web3::types::Bytes(vec![]),
    };
    let options = Options {
        gas: Some(U256::from_dec_str("10000000")?),
        ..Default::default()
    };
    let num = 10;
    for _ in 0..num {
        let tx = msg
            .source
            .outbound
            .send_message(
                message.clone(),
                msg.source.private_key(),
                fee,
                options.clone(),
            )
            .await
            .unwrap();
        wait_for_transaction_confirmation(
            tx,
            msg.source.get_web3().transport(),
            Duration::from_secs(3),
            1,
        )
        .await?;
        dbg!(&tx);
    }
    Ok(())
}

#[tokio::test]
async fn test_msg_eth_to_darwinia() -> color_eyre::Result<()> {
    let msg = message_relay_client_builder().await?;
    // Get fee from fee market
    let relayer_info = msg.target.strategy.fee_market.get_relayer_info().await?;
    dbg!(&relayer_info);
    let fee = relayer_info.fee;
    dbg!(fee);
    // Send messages
    let message = SendMessage {
        target_contract: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        encoded: web3::types::Bytes(vec![]),
    };
    let options = Options {
        gas: Some(U256::from_dec_str("10000000")?),
        gas_price: Some(U256::from_dec_str("20000000000")?),
        ..Default::default()
    };
    for _ in 0..10 {
        let tx = msg
            .target
            .outbound
            .send_message(
                message.clone(),
                msg.target.private_key(),
                fee,
                options.clone(),
            )
            .await
            .unwrap();
        dbg!(&tx);
        wait_for_transaction_confirmation(
            tx,
            msg.target.get_web3().transport(),
            Duration::from_secs(3),
            1,
        )
        .await?;
    }
    Ok(())
}
