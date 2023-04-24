use std::{str::FromStr, time::Duration};

use bin_e2e::{
    config::BridgeConfig, service::message_relay::darwinia_to_eth::message_relay_client_builder,
};
use bridge_e2e_traits::client::{MessageClient, Web3Client};
use bridge_pangolin_goerli::bridge::BridgeConfig as RawBridgeConfig;
use client_contracts::outbound_types::SendMessage;
use client_pangolin::client::PangolinClient;
use relay_e2e::types::ethereum::FastEthereumAccount;
use secp256k1::SecretKey;
use subquery::types::BridgeName;
use support_common::config::{Config, Names};
use support_etherscan::wait_for_transaction_confirmation_with_timeout;
use thegraph::types::LikethChain;
use web3::{
    contract::{tokens::Tokenize, Options},
    ethabi::Address,
    signing::Key,
    types::U256,
};

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

async fn get_bridge_config() -> color_eyre::Result<BridgeConfig<PangolinClient>> {
    let raw_config: RawBridgeConfig = Config::restore(Names::BridgePangolinGoerli)?;
    let bridge_config = BridgeConfig {
        name: Names::BridgePangolinGoerli.name().into(),
        general: raw_config.general,
        darwinia_evm: raw_config.pangolin_evm,
        substrate_client: raw_config.pangolin_substrate.to_substrate_client().await?,
        ethereum: raw_config.goerli,
        beacon: raw_config.beacon,
        substrate_index: raw_config
            .index
            .to_substrate_subquery(BridgeName::PangolinGoerli),
        evm_index: raw_config.index.to_evm_thegraph(LikethChain::Pangolin)?,
    };
    Ok(bridge_config)
}

#[ignore]
#[tokio::test]
async fn test_enroll_relayer_at_pangolin() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config).await?;
    let privates = vec![
        "40b50cd43ccbfe7da7e594216710eac2ab0036fa59a957a85c5d8ee4f3761f49",
        // "eb67cea5965fb74aa9fd439f746444dd69cef8d6164af86c04d259f2f35799e8",
        // "8111947fa1a4c5d7c2b81e66f930e3dd08e2001aa5f4e316301395b9e0423206",
    ];
    let mut prev = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    for key in privates {
        let secret = SecretKey::from_str(&key)?;
        let address = (&secret).address();
        dbg!(&address);
        let tx = msg
            .source
            .strategy
            .fee_market
            .enroll(
                prev,
                U256::from_dec_str("10000000000000000000").unwrap(),
                &secret,
            )
            .await
            .unwrap();
        wait_for_transaction_confirmation_with_timeout(
            tx,
            msg.source.get_web3().transport(),
            Duration::from_secs(3),
            1,
            1
        )
        .await?;
        prev = Address::from(address);
        dbg!(tx);
    }
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_enroll_relayer_at_goerli() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config.clone()).await?;
    let mut prev = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let mut count = 0;
    loop {
        if count > 3 {
            break;
        }
        let relayer = msg.target.strategy.fee_market.get_relayer(prev).await?;
        dbg!(&relayer);
        prev = relayer;
        count += 1;
    }
    let min: U256 = msg
        .target
        .strategy
        .fee_market
        .contract
        .query("COLLATERAL_PER_ORDER", (), None, Options::default(), None)
        .await
        .unwrap();
    dbg!(min);
    let balance = msg
        .target
        .strategy
        .fee_market
        .balance_of(Address::from_str("0x68898dB1012808808C903F390909C52D9F706749").unwrap())
        .await?;
    dbg!(balance);

    // let relayer_info = msg.target.strategy.fee_market.get_relayer_info().await?;
    // dbg!(&relayer_info);
    // let fee = relayer_info.get(0).expect("There are no relayers!").fee;
    // dbg!(fee);

    let mut prev = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let secret = SecretKey::from_str(&config.ethereum.private_key)?;
    let addresss = (&secret).address();
    dbg!(addresss);
    let tx = msg
        .target
        .strategy
        .fee_market
        .enroll(
            prev,
            U256::from_dec_str("100000000000000").unwrap(),
            &secret,
        )
        .await
        .unwrap();
    dbg!(tx);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_deposit_relayer_pangolin() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config).await?;
    let privates = vec![
        "40b50cd43ccbfe7da7e594216710eac2ab0036fa59a957a85c5d8ee4f3761f49",
        // "eb67cea5965fb74aa9fd439f746444dd69cef8d6164af86c04d259f2f35799e8",
        // "8111947fa1a4c5d7c2b81e66f930e3dd08e2001aa5f4e316301395b9e0423206",
    ];
    for key in privates {
        let secret = SecretKey::from_str(&key)?;
        let tx = msg
            .source
            .strategy
            .fee_market
            .deposit(
                U256::from_dec_str("1000000000000000000000").unwrap(),
                &secret,
            )
            .await
            .unwrap();
        dbg!(tx);
    }
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_deposit_relayer_goerli() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config.clone()).await?;
    let secret = SecretKey::from_str(&config.ethereum.private_key)?;
    let tx = msg
        .target
        .strategy
        .fee_market
        .deposit(U256::from(2100000000000000u64), &secret)
        .await
        .unwrap();
    dbg!(tx);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_msg_darwinia_to_eth() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config).await?;
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
        gas: Some(U256::from_dec_str("8000000")?),
        ..Default::default()
    };
    let num = 1;
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
        // wait_for_transaction_confirmation(
        //     tx,
        //     msg.source.get_web3().transport(),
        //     Duration::from_secs(3),
        //     1,
        // )
        // .await?;
        dbg!(&tx);
    }
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_msg_eth_to_darwinia() -> color_eyre::Result<()> {
    let config = get_bridge_config().await?;
    let msg = message_relay_client_builder(config).await?;
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
        gas: Some(U256::from_dec_str("1000000")?),
        gas_price: None,
        ..Default::default()
    };
    for _ in 0..1 {
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
        wait_for_transaction_confirmation_with_timeout(
            tx,
            msg.target.get_web3().transport(),
            Duration::from_secs(3),
            1,
            1
        )
        .await?;
    }
    Ok(())
}
