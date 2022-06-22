use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use abstract_client_s2s::client::S2SClientRelay;
use abstract_client_s2s::error::S2SClientError;
use once_cell::sync::Lazy;

use crate::error::RelayResult;
use crate::types::JustificationInput;

static RECENTLY_JUSTIFICATIONS: Lazy<Mutex<HashMap<&str, VecDeque<sp_core::Bytes>>>> =
    Lazy::new(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });

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
            let mut data = RECENTLY_JUSTIFICATIONS.lock().unwrap();
            let queue = data
                .entry(SC::CHAIN)
                .or_insert_with(|| VecDeque::with_capacity(100));
            queue.push_back(justification);
            if queue.len() >= 100 {
                queue.pop_front();
            }
        }));
        let join_b = tokio::spawn(run_until_connection_lost(client_target, |justification| {
            let mut data = RECENTLY_JUSTIFICATIONS.lock().unwrap();
            let queue = data
                .entry(TC::CHAIN)
                .or_insert_with(|| VecDeque::with_capacity(100));
            queue.push_back(justification);
            if queue.len() >= 100 {
                queue.pop_front();
            }
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
            target: "relay-s2s", "[subscribe] [{}] Failed to get justification from {}: {:?}",
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
        callback(justification);
    }
    Ok(())
}

pub(crate) fn recently_justification(
    chain: impl AsRef<str>,
) -> RelayResult<Option<sp_core::Bytes>> {
    let recently_justifications = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let justification_queue = recently_justifications.get(chain.as_ref());
    match justification_queue {
        Some(v) => Ok(v.back().cloned()),
        None => Ok(None),
    }
}
