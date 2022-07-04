use std::time::Duration;

use bridge_s2s_traits::client::S2SClientGeneric;
use bridge_s2s_traits::error::S2SClientError;
use jsonrpsee_core::client::Subscription;

use support_toolkit::logk;

use crate::error::{RelayError, RelayResult};
use crate::keepstate;
use crate::types::JustificationInput;

pub struct SubscribeJustification<C: S2SClientGeneric> {
    input: JustificationInput<C>,
}

impl<C: S2SClientGeneric> SubscribeJustification<C> {
    pub fn new(input: JustificationInput<C>) -> Self {
        Self { input }
    }
}

impl<C: S2SClientGeneric> SubscribeJustification<C> {
    pub async fn start(self) -> RelayResult<()> {
        let client = self.input.client;
        let join_a = tokio::spawn(run_until_connection_lost(client, |justification| {
            keepstate::set_recently_justification(C::CHAIN, justification);
        }));
        join_a
            .await
            .map_err(|e| S2SClientError::RPC(format!("{:?}", e)))??;
        Ok(())
    }
}

async fn run_until_connection_lost<T, F>(client: T, callback: F) -> RelayResult<()>
where
    T: S2SClientGeneric,
    F: Send + Sync + Fn(sp_core::Bytes),
{
    if let Err(err) = subscribe_justification(&client, callback).await {
        tracing::error!(
            target: "relay-s2s",
            "{} failed to get justification from {}: {:?}",
            logk::prefix_multi("subscribe", vec![T::CHAIN]),
            T::CHAIN,
            err,
        );
        return Err(err);
    }
    Err(RelayError::Custom(format!(
        "[{}] the subscription stopped",
        T::CHAIN,
    )))
}

async fn subscribe_justification<T, F>(client: &T, callback: F) -> RelayResult<()>
where
    T: S2SClientGeneric,
    F: Send + Sync + Fn(sp_core::Bytes),
{
    let mut subscribe = client.subscribe_grandpa_justifications().await?;
    let timeout = std::time::Duration::from_secs(30);
    loop {
        let next_justification = safe_read_justification(timeout, &mut subscribe).await?;
        match next_justification {
            Some(justification) => {
                let justification =
                    justification.map_err(|e| S2SClientError::RPC(format!("{:?}", e)))?;
                tracing::info!(
                    target: "relay-s2s",
                    "{} subscribed new justification for {}",
                    logk::prefix_multi("subscribe", vec![T::CHAIN]),
                    T::CHAIN,
                );
                callback(justification);
            }
            None => {
                tracing::error!(
                    target: "relay-s2s",
                    "{} the subscription has been terminated",
                    logk::prefix_multi("subscribe", vec![T::CHAIN]),
                );
                return Err(RelayError::Custom(format!(
                    "[{}] the subscription has been terminated",
                    T::CHAIN,
                )));
            }
        }
    }
}

async fn safe_read_justification(
    timeout: Duration,
    subscribe: &mut Subscription<sp_core::Bytes>,
) -> RelayResult<Option<Result<sp_core::Bytes, jsonrpsee_core::Error>>> {
    let timeout = tokio::time::sleep(timeout);
    tokio::select! {
        res = subscribe.next() => Ok(res),
        _ = timeout => Err(RelayError::Custom("subscribe timeout".to_string()))
    }
}
