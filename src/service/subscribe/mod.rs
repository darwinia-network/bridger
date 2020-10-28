//! Darwinia Subscribe
use crate::{
    api::{Darwinia, Shadow},
    result::Result as BridgerResult,
    service::Service,
    Pool,
};
use async_trait::async_trait;
use primitives::{
    frame::ethereum::{
        backing::EthereumBackingEventsDecoder, game::EthereumRelayerGameEventsDecoder,
        relay::EthereumRelayEventsDecoder,
    },
    runtime::DarwiniaRuntime,
};
use std::sync::{Arc, Mutex};
use substrate_subxt::{EventSubscription, EventsDecoder};

mod backing;
mod relay;

/// Attributes
const SERVICE_NAME: &str = "SUBSCRIBE";
const ETHEREUM_RELAY: &str = "EthereumRelay";
const ETHEREUM_BACKING: &str = "EthereumBacking";

/// Dawrinia Subscribe
pub struct SubscribeService {
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
}

impl SubscribeService {
    /// New redeem service
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> SubscribeService {
        SubscribeService { darwinia, shadow }
    }
}

#[async_trait(?Send)]
impl Service for SubscribeService {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, _: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        let client = &self.darwinia.client;
        let scratch = client.subscribe_events().await?;
        let mut decoder = EventsDecoder::<DarwiniaRuntime>::new(client.metadata().clone());

        // Register decoders
        decoder.with_ethereum_backing();
        decoder.with_ethereum_relayer_game();
        decoder.with_ethereum_relay();

        // Build subscriber
        let mut sub = EventSubscription::<DarwiniaRuntime>::new(scratch, decoder);
        if let Some(raw) = sub.next().await {
            if let Ok(event) = raw {
                match event.module.as_str() {
                    ETHEREUM_RELAY => relay::handle(event)?,
                    ETHEREUM_BACKING => backing::handle(event)?,
                    _ => {}
                };
            }
        }

        Ok(())
    }
}
