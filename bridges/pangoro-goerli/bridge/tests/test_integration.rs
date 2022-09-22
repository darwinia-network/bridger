use std::{str::FromStr, time::Duration};

use bridge_pangoro_goerli::{
    bridge::BridgeConfig,
    goerli_client::client::GoerliClient,
    message_contract::{
        darwinia_message_client::{build_darwinia_message_client, DarwiniaMessageClient},
        fee_market::FeeMarketRelayStrategy,
        message_client::{build_message_client_with_simple_fee_market, MessageClient},
        simple_fee_market::SimpleFeeMarketRelayStrategy,
    },
    pangoro_client::client::PangoroClient,
    service::header_relay::{
        execution_layer_update::ExecutionLayer, goerli_to_pangoro::HeaderRelay,
        sync_committee_update::SyncCommitteeUpdate,
    },
    web3_helper::wait_for_transaction_confirmation,
};
use client_contracts::{outbound_types::SendMessage, PosaLightClient, SimpleFeeMarket};
use support_common::config::{Config, Names};
use web3::types::{Address, U256};

fn build_goerli_to_pangoro_header_realyer() -> color_eyre::Result<HeaderRelay> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        config.pangoro_evm.gas_option(),
    )?;
    let goerli_client = GoerliClient::new(&config.goerli.endpoint)?;
    let header_relay = HeaderRelay {
        pangoro_client,
        goerli_client,
    };
    Ok(header_relay)
}

fn build_execution_relayer() -> color_eyre::Result<ExecutionLayer> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        config.pangoro_evm.gas_option(),
    )?;
    let goerli_client = GoerliClient::new(&config.goerli.endpoint)?;
    let execution_layer_relay = ExecutionLayer {
        pangoro_client,
        goerli_client,
    };
    Ok(execution_layer_relay)
}

fn build_sync_committee_relayer() -> color_eyre::Result<SyncCommitteeUpdate> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        config.pangoro_evm.gas_option(),
    )?;
    let goerli_client = GoerliClient::new(&config.goerli.endpoint)?;
    Ok(SyncCommitteeUpdate {
        pangoro_client,
        goerli_client,
    })
}

fn build_message_clients() -> color_eyre::Result<(
    MessageClient<SimpleFeeMarketRelayStrategy>,
    DarwiniaMessageClient<FeeMarketRelayStrategy>,
    PangoroClient,
    GoerliClient,
    PosaLightClient,
)> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let beacon_light_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        config.pangoro_evm.gas_option(),
    )?;
    let beacon_rpc_client = GoerliClient::new(&config.goerli.endpoint)?;
    let goerli = build_message_client_with_simple_fee_market(
        &config.goerli.execution_layer_endpoint,
        Address::from_str(&config.goerli.inbound_address)?,
        Address::from_str(&config.goerli.outbound_address)?,
        Address::from_str(&config.goerli.fee_market_address)?,
        Address::from_str(&config.goerli.account)?,
        Some(&config.goerli.private_key),
        config.goerli.gas_option(),
    )
    .unwrap();
    let darwinia = build_darwinia_message_client(
        &config.pangoro_evm.endpoint,
        Address::from_str(&config.pangoro_evm.inbound_address)?,
        Address::from_str(&config.pangoro_evm.outbound_address)?,
        Address::from_str(&config.pangoro_evm.chain_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.lane_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.fee_market_address)?,
        Address::from_str(&config.pangoro_evm.account)?,
        Some(&config.pangoro_evm.private_key),
        config.index.to_pangoro_thegraph()?,
        config.pangoro_evm.gas_option(),
    )
    .unwrap();
    let posa_light_client = PosaLightClient::new(
        goerli.client.clone(),
        Address::from_str(&config.goerli.posa_light_client_address)?,
    )?;
    Ok((
        goerli,
        darwinia,
        beacon_light_client,
        beacon_rpc_client,
        posa_light_client,
    ))
}

#[tokio::test]
async fn test_messages_from_pangoro_to_goerli() -> color_eyre::Result<()> {
    let header_relay = build_goerli_to_pangoro_header_realyer()?;
    let execution_layer_relayer = build_execution_relayer()?;
    let sync_committee_relayer = build_sync_committee_relayer()?;
    let (goerli, darwinia, beacon_light_client, beacon_rpc_client, posa_light_client) =
        build_message_clients()?;

    let message = SendMessage {
        target_contract: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        encoded: web3::types::Bytes(vec![]),
    };
    let goerli_relayer = goerli.strategy.fee_market.get_top_relayer().await?;
    let goerli_fee = goerli.strategy.fee_market.fee_of(goerli_relayer).await?;
    dbg!(&goerli_fee);
    let fee = U256::from_dec_str("9000000000000000000")?;
    for _ in 0..20 {
        let g_tx = goerli
            .outbound
            .send_message(
                message.clone(),
                &goerli.private_key()?,
                goerli_fee,
                goerli.gas_option.clone(),
            )
            .await?;
        let g_transport = goerli.client.transport().clone();
        dbg!(g_tx);

        let tx = darwinia
            .outbound
            .send_message(
                message.clone(),
                &darwinia.private_key()?,
                fee,
                darwinia.gas_option.clone(),
            )
            .await?;
        let transport = darwinia.client.transport().clone();
        dbg!(tx);
        wait_for_transaction_confirmation(g_tx, g_transport, Duration::from_secs(2), 2).await?;
        wait_for_transaction_confirmation(tx, transport, Duration::from_secs(2), 2).await?;
    }
    // goerli.outbound.contract.signed_call_with_confirmations(
    //     func,
    //     params,
    //     options,
    //     confirmations,
    //     key,
    // );
    Ok(())
}
