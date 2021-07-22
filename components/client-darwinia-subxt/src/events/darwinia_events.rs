use substrate_subxt::{
    events::Raw, sp_core::Decode, Client, EventTypeRegistry, EventsDecoder, RawEvent,
};

use crate::darwinia::runtime::DarwiniaRuntime;
use crate::error::Result;
use crate::frame::bridge::relay_authorities::{
    AuthoritiesChangeSigned, EthereumRelayAuthorities, MMRRootSigned, ScheduleAuthoritiesChange,
    ScheduleMMRRoot,
};

/// Darwinia Event Info
pub enum EventInfo<T: EthereumRelayAuthorities> {
    MMRRootSignedEvent(MMRRootSigned<T>),
    ScheduleMMRRootEvent(ScheduleMMRRoot<T>),
    ScheduleAuthoritiesChangeEvent(ScheduleAuthoritiesChange<T>),
    AuthoritiesChangeSignedEvent(AuthoritiesChangeSigned<T>),

    RuntimeUpdatedEvent(String),
    Invalid(String),
}

/// Darwinia Events
pub struct DarwiniaEvents {
    /// event decoder
    pub decoder: EventsDecoder<DarwiniaRuntime>,
    client: Client<DarwiniaRuntime>,
}

impl Clone for DarwiniaEvents {
    fn clone(&self) -> Self {
        DarwiniaEvents::new(self.client.clone())
    }
}

impl DarwiniaEvents {
    pub fn new(client: Client<DarwiniaRuntime>) -> Self {
        let event_type_registry = EventTypeRegistry::<DarwiniaRuntime>::new();
        let decoder =
            EventsDecoder::<DarwiniaRuntime>::new(client.metadata().clone(), event_type_registry);
        DarwiniaEvents { decoder, client }
    }

    pub fn decode_events(&self, input: &mut &[u8]) -> Result<Vec<RawEvent>> {
        let raw_events = self.decoder.decode_events(input)?;
        let mut events = vec![];
        for (_, raw) in raw_events {
            match raw {
                Raw::Event(event) => {
                    events.push(event);
                }
                Raw::Error(err) => {
                    log::error!("Error found in raw events: {:#?}", err);
                }
            }
        }
        Ok(events)
    }

    /// parse event
    pub fn parse_event(
        &self,
        module: &str,
        variant: &str,
        event_data: Vec<u8>,
    ) -> EventInfo<DarwiniaRuntime> {
        match (module, variant) {
            ("System", "CodeUpdated") => {
                return EventInfo::RuntimeUpdatedEvent("code updated".to_string());
            }
            ("EthereumRelayAuthorities", "ScheduleMMRRoot") => {
                if let Ok(decoded) =
                    ScheduleMMRRoot::<DarwiniaRuntime>::decode(&mut &event_data[..])
                {
                    return EventInfo::ScheduleMMRRootEvent(decoded);
                }
            }
            ("EthereumRelayAuthorities", "MMRRootSigned") => {
                MMRRootSigned::<DarwiniaRuntime>::decode(&mut &event_data[..]).unwrap();
                if let Ok(decoded) = MMRRootSigned::<DarwiniaRuntime>::decode(&mut &event_data[..])
                {
                    return EventInfo::MMRRootSignedEvent(decoded);
                }
            }
            ("EthereumRelayAuthorities", "ScheduleAuthoritiesChange") => {
                if let Ok(decoded) =
                    ScheduleAuthoritiesChange::<DarwiniaRuntime>::decode(&mut &event_data[..])
                {
                    return EventInfo::ScheduleAuthoritiesChangeEvent(decoded);
                }
            }
            ("EthereumRelayAuthorities", "AuthoritiesChangeSigned") => {
                if let Ok(decoded) =
                    AuthoritiesChangeSigned::<DarwiniaRuntime>::decode(&mut &event_data[..])
                {
                    return EventInfo::AuthoritiesChangeSignedEvent(decoded);
                }
            }
            _ => {
                return EventInfo::Invalid(String::from(module) + "::" + variant);
            }
        }
        EventInfo::Invalid(String::from(module) + "::" + variant)
    }
}
