use abstract_bridge_s2s::client::S2SClientRelay;
use abstract_bridge_s2s::error::S2SClientError;

use crate::error::RelayResult;
use crate::keepstate;
use crate::types::JustificationInput;

pub struct SubscribeJustification<SC: S2SClientRelay, TC: S2SClientRelay> {
    input: JustificationInput<SC, TC>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> SubscribeJustification<SC, TC> {
    pub fn new(input: JustificationInput<SC, TC>) -> Self {
        Self { input }
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> SubscribeJustification<SC, TC> {
    pub async fn start(self) -> RelayResult<()> {
        let client_source = self.input.client_source;
        let client_target = self.input.client_target;
        let join_a = tokio::spawn(run_until_connection_lost(client_source, |justification| {
            keepstate::set_recently_justification(SC::CHAIN, justification);
        }));
        let join_b = tokio::spawn(run_until_connection_lost(client_target, |justification| {
            keepstate::set_recently_justification(TC::CHAIN, justification);
        }));
        let (_result_a, _result_b) = (
            join_a
                .await
                .map_err(|e| S2SClientError::RPC(format!("{:?}", e)))?,
            join_b
                .await
                .map_err(|e| S2SClientError::RPC(format!("{:?}", e)))?,
        );
        Ok(())
    }
}

async fn run_until_connection_lost<T, F>(client: T, callback: F) -> RelayResult<()>
where
    T: S2SClientRelay,
    F: Send + Sync + Fn(sp_core::Bytes),
{
    if let Err(err) = subscribe_justification(&client, callback).await {
        tracing::error!(
            target: "relay-s2s",
            "[subscribe] [{}] Failed to get justification from {}: {:?}",
            T::CHAIN,
            T::CHAIN,
            err
        );
    }
    Ok(())
}

async fn subscribe_justification<T, F>(client: &T, callback: F) -> RelayResult<()>
where
    T: S2SClientRelay,
    F: Send + Sync + Fn(sp_core::Bytes),
{
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    while let Some(justification) = subscribe.next().await {
        let justification = justification.map_err(|e| S2SClientError::RPC(format!("{:?}", e)))?;
        tracing::info!(
            target: "relay-s2s",
            "[subscribe] subscribed new justification for {}",
            T::CHAIN,
        );
        callback(justification);
    }
    Ok(())
}
