//! Darwinia Subscribe
use crate::{
    api::{Darwinia, Shadow},
    result::Result as BridgerResult,
    result::Error,
};
use primitives::{
    frame::ethereum::{
        backing::EthereumBackingEventsDecoder, game::EthereumRelayerGameEventsDecoder,
        relay::EthereumRelayEventsDecoder,
    },
    runtime::DarwiniaRuntime,
};
use std::sync::Arc;
use substrate_subxt::{EventSubscription, EventsDecoder};

mod backing;
mod relay;

/// Attributes
const ETHEREUM_RELAY: &str = "EthereumRelay";
const ETHEREUM_BACKING: &str = "EthereumBacking";

/// Dawrinia Subscribe
pub struct SubscribeService {
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,

    sub: EventSubscription<DarwiniaRuntime>,
}

impl SubscribeService {
    /// New redeem service
    pub async fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> BridgerResult<SubscribeService> {
        let sub = SubscribeService::build_event_subscription(darwinia.clone()).await?;
        Ok(SubscribeService {
            darwinia,
            shadow,
            sub
        })
    }

    /// start
    pub async fn start(&mut self) -> BridgerResult<()> {
        info!("🌟 SERVICE STARTED: SUBSCRIBE");
        loop {
            if let Err(e) = self.process_next_event().await {
                if &e.to_string() != "CodeUpdated" {
                    error!("Fail to process next event: {:?}", e);
                } else {
                    return Err(e);
                }
            }
        }
    }

    /// process_next_event
    async fn process_next_event(&mut self) -> BridgerResult<()> {
        if let Some(raw) = self.sub.next().await {
            if let Ok(event) = raw {
                // Remove the system events temporarily because it`s too verbose.
                if &event.module == "System" {
                    if event.variant.as_str() == "CodeUpdated" {
                        return Err(Error::Bridger("CodeUpdated".to_string()));
                    }
                } else {
                    // Common events to debug
                    debug!(">> Event - {}::{}", &event.module, &event.variant);
                    match event.module.as_str() {
                        ETHEREUM_RELAY => relay::handle(event)?,
                        ETHEREUM_BACKING => backing::handle(event)?,
                        _ => {}
                    };
                }
            }
        }
        Ok(())
    }

    async fn build_event_subscription(darwinia: Arc<Darwinia>) -> BridgerResult<EventSubscription<DarwiniaRuntime>> {
        let client = &darwinia.client;
        let scratch = client.subscribe_events().await?;
        let mut decoder = EventsDecoder::<DarwiniaRuntime>::new(client.metadata().clone());

        // Register decoders
        decoder.with_ethereum_backing();
        decoder.with_ethereum_relayer_game();
        decoder.with_ethereum_relay();

        // Build subscriber
        let sub = EventSubscription::<DarwiniaRuntime>::new(scratch, decoder);
        Ok(sub)
    }
}
