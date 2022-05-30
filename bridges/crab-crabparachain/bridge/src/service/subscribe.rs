use client_pangolin::{client::PangolinClient, component::PangolinClientComponent};
use client_kusama::{client::KusamaClient, component::KusamaClientComponent};
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

pub static ROCOCO_JUSTIFICATIONS: Lazy<Mutex<VecDeque<sp_core::Bytes>>> = Lazy::new(|| {
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
            let mut execution = start().await;
            while let Err(e) = execution {
                tracing::error!(target: "pangolin-crabparachain", "{:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(target: "pangolin-crabparachain", "Try to restart subscribtion service.");
                execution = start().await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinCrabParachain)?;
    let config_pangolin = bridge_config.pangolin;

    let client_pangolin =
        PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?).await?;
    let client_kusama =
        KusamaClientComponent::component(bridge_config.kusama.to_kusama_client_config()?).await?;

    let pangolin_handle = tokio::spawn(run_until_pangolin_connection_lost(client_pangolin));
    let kusama_handle = tokio::spawn(run_until_kusama_connection_lost(client_kusama));
    let (_result_p, _result_r) = (pangolin_handle.await, kusama_handle.await);
    Ok(())
}

async fn run_until_pangolin_connection_lost(mut client: PangolinClient) -> color_eyre::Result<()> {
    while let Err(err) = subscribe_pangolin(&client).await {
        tracing::error!(target: "pangolin-crabparachain", "Failed to get justification from pangolin: {:?}", err);
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinCrabParachain)?;
        let client_pangolin =
            PangolinClientComponent::component(bridge_config.pangolin.to_pangolin_client_config()?)
                .await?;
        client = client_pangolin;
    }
    Ok(())
}

async fn run_until_kusama_connection_lost(mut client: KusamaClient) -> color_eyre::Result<()> {
    while let Err(err) = subscribe_kusama(&client).await {
        tracing::error!(target: "pangolin-crabparachain", "Failed to get justification from kusama: {:?}", err);
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinCrabParachain)?;
        let client_kusama =
            KusamaClientComponent::component(bridge_config.kusama.to_kusama_client_config()?)
                .await?;
        client = client_kusama;
    }
    Ok(())
}

async fn subscribe_pangolin(client: &PangolinClient) -> color_eyre::Result<()> {
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    while let Some(justification) = subscribe.next().await {
        let mut data = PANGOLIN_JUSTIFICATIONS.lock().await;
        data.push_back(justification.unwrap());
        if data.len() >= 100 {
            data.pop_front();
        }
    }
    Ok(())
}

async fn subscribe_kusama(client: &KusamaClient) -> color_eyre::Result<()> {
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    while let Some(justification) = subscribe.next().await {
        let mut data = ROCOCO_JUSTIFICATIONS.lock().await;
        data.push_back(justification.unwrap());
        if data.len() >= 100 {
            data.pop_front();
        }
    }
    Ok(())
}
