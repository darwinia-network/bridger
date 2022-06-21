use client_pangolin::{client::PangolinClient, component::PangolinClientComponent};
use client_pangoro::{client::PangoroClient, component::PangoroClientComponent};
use lifeline::{Lifeline, Service, Task};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
// use std::sync::Mutex;
use futures::lock::Mutex;
// use subxt::rpc::Subscription;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

pub static PANGOLIN_JUSTIFICATIONS: Lazy<Mutex<VecDeque<sp_core::Bytes>>> = Lazy::new(|| {
    let d = VecDeque::with_capacity(100);
    Mutex::new(d)
});

pub static PANGORO_JUSTIFICATIONS: Lazy<Mutex<VecDeque<sp_core::Bytes>>> = Lazy::new(|| {
    let d = VecDeque::with_capacity(100);
    Mutex::new(d)
});

#[derive(Debug)]
pub struct SubscribeService {
    _greet: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(&format!("{}-relay", BridgeTask::name()), async move {
            while let Err(e) = start().await {
                tracing::error!(target: "pangolin-pangoro", "[subscribe] Failed to start subscribe {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(target: "pangolin-pangoro", "[subscribe] Try to restart subscription service.");
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_pangoro = bridge_config.pangoro.to_pangoro_client().await?;

    let pangolin_handle = tokio::spawn(run_until_pangolin_connection_lost(client_pangolin));
    let pangoro_handle = tokio::spawn(run_until_pangoro_connection_lost(client_pangoro));
    let (_result_p, _result_r) = (pangolin_handle.await, pangoro_handle.await);
    Ok(())
}

async fn run_until_pangolin_connection_lost(mut client: PangolinClient) -> color_eyre::Result<()> {
    while let Err(err) = subscribe_pangolin(&client).await {
        tracing::error!(target: "pangolin-pangoro", "[subscribe] [pangolin] Failed to get justification from pangolin: {:?}", err);
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
        client = bridge_config.pangolin.to_pangolin_client().await?;
    }
    Ok(())
}

async fn run_until_pangoro_connection_lost(mut client: PangoroClient) -> color_eyre::Result<()> {
    while let Err(err) = subscribe_pangoro(&client).await {
        tracing::error!(target: "pangolin-pangoro", "[subscribe] [pangoro] Failed to get justification from pangoro: {:?}", err);
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
        client = bridge_config.pangoro.to_pangoro_client().await?;
    }
    Ok(())
}

async fn subscribe_pangolin(client: &PangolinClient) -> color_eyre::Result<()> {
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    while let Some(justification) = subscribe.next().await {
        let mut data = PANGOLIN_JUSTIFICATIONS.lock().await;
        data.push_back(justification?);
        if data.len() >= 100 {
            data.pop_front();
        }
    }
    Ok(())
}

async fn subscribe_pangoro(client: &PangoroClient) -> color_eyre::Result<()> {
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    while let Some(justification) = subscribe.next().await {
        let mut data = PANGORO_JUSTIFICATIONS.lock().await;
        data.push_back(justification?);
        if data.len() >= 100 {
            data.pop_front();
        }
    }
    Ok(())
}
