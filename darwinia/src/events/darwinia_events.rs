use substrate_subxt::{events::Raw, sp_core::Decode, Client, EventTypeRegistry, EventsDecoder, RawEvent, Runtime};

use crate::error::Result;

//TODO move here
use primitives::{
	frame::bridge::relay_authorities::{
		AuthoritiesChangeSigned, EthereumRelayAuthorities, MMRRootSigned,
		ScheduleAuthoritiesChange, ScheduleMMRRoot,
	},
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
pub struct DarwiniaEvents<R: Runtime + EthereumRelayAuthorities> {
	/// event decoder
	pub decoder: EventsDecoder<R>,
	client: Client<R>,
}

impl<R: Runtime + EthereumRelayAuthorities> Clone for DarwiniaEvents<R> {
	fn clone(&self) -> Self {
		DarwiniaEvents::<R>::new(self.client.clone())
	}
}

impl<R: Runtime + EthereumRelayAuthorities> DarwiniaEvents<R> {
	pub fn new(client: Client<R>) -> Self {
		let event_type_registry = EventTypeRegistry::<R>::new();
		let decoder =
			EventsDecoder::<R>::new(client.metadata().clone(), event_type_registry);
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
					error!("Error found in raw events: {:#?}", err);
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
	) -> EventInfo<R> {
		match (module, variant) {
			("System", "CodeUpdated") => {
				return EventInfo::RuntimeUpdatedEvent("code updated".to_string());
			}
			("EthereumRelayAuthorities", "ScheduleMMRRoot") => {
				if let Ok(decoded) =
					ScheduleMMRRoot::<R>::decode(&mut &event_data[..])
				{
					return EventInfo::ScheduleMMRRootEvent(decoded);
				}
			}
			("EthereumRelayAuthorities", "MMRRootSigned") => {
				MMRRootSigned::<R>::decode(&mut &event_data[..]).unwrap();
				if let Ok(decoded) = MMRRootSigned::<R>::decode(&mut &event_data[..])
				{
					return EventInfo::MMRRootSignedEvent(decoded);
				}
			}
			("EthereumRelayAuthorities", "ScheduleAuthoritiesChange") => {
				if let Ok(decoded) =
					ScheduleAuthoritiesChange::<R>::decode(&mut &event_data[..])
				{
					return EventInfo::ScheduleAuthoritiesChangeEvent(decoded);
				}
			}
			("EthereumRelayAuthorities", "AuthoritiesChangeSigned") => {
				if let Ok(decoded) =
					AuthoritiesChangeSigned::<R>::decode(&mut &event_data[..])
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
