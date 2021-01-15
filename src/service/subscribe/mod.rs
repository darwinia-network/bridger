//! Darwinia Subscribe
mod darwinia_tracker;

use crate::api::Ethereum;
use crate::error::{BizError, Error};
use crate::service::subscribe::darwinia_tracker::DarwiniaBlockTracker;
use crate::tools;
use crate::{
	api::Darwinia,
	error::Result,
	service::extrinsics::{Extrinsic, MsgExtrinsic},
};
use actix::Recipient;
use primitives::{
	frame::bridge::relay_authorities::{
		AuthoritiesChangeSigned, ScheduleAuthoritiesChange, ScheduleMMRRoot,
	},
	runtime::DarwiniaRuntime,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use substrate_subxt::sp_core::Decode;
use substrate_subxt::system::System;
use substrate_subxt::RawEvent;
use tokio::time::{delay_for, Duration};

/// Dawrinia Subscribe
pub struct SubscribeService {
	darwinia: Arc<Darwinia>,
	ethereum: Ethereum,
	stop: bool,
	extrinsics_service: Recipient<MsgExtrinsic>,
	delayed_extrinsics: HashMap<u32, Extrinsic>,
	spec_name: String,
	scan_from: u32,
	data_dir: PathBuf,
}

impl SubscribeService {
	/// New subscribe service
	pub fn new(
		darwinia: Arc<Darwinia>,
		ethereum: Ethereum,
		extrinsics_service: Recipient<MsgExtrinsic>,
		spec_name: String,
		scan_from: u32,
		data_dir: PathBuf,
	) -> SubscribeService {
		SubscribeService {
			darwinia,
			ethereum,
			stop: false,
			extrinsics_service,
			delayed_extrinsics: HashMap::new(),
			spec_name,
			scan_from,
			data_dir,
		}
	}

	/// start
	pub async fn start(&mut self) -> Result<()> {
		let mut tracker = DarwiniaBlockTracker::new(self.darwinia.clone(), self.scan_from);
		info!("✨ SERVICE STARTED: SUBSCRIBE");
		loop {
			let header = tracker.next_block().await;

			// debug
			trace!("Darwinia block {}", header.number);

			// handle the 'mmr root sign and send extrinsics' only block height reached
			if let Err(err) = self.handle_delayed_extrinsics(&header).await {
				error!(
					"An error occurred while processing the delayed extrinsics: {:?}",
					err
				);
				// Prevent too fast refresh errors
				delay_for(Duration::from_secs(30)).await;
			}

			// handle events of the block
			let hash = header.hash();
			let events = self.darwinia.get_raw_events(hash).await;
			if let Err(err) = self.handle_events(&header, events).await {
				if let Some(Error::RuntimeUpdated) = err.downcast_ref() {
					tools::set_cache(
						self.data_dir.clone(),
						tools::LAST_TRACKED_ETHEREUM_BLOCK_FILE_NAME,
						header.number as u64,
					)
					.await?;
					return Err(err);
				} else {
					error!(
						"An error occurred while processing the events of block {}: {:?}",
						header.number, err
					);
				}
			}

			tools::set_cache(
				self.data_dir.clone(),
				tools::LAST_TRACKED_ETHEREUM_BLOCK_FILE_NAME,
				header.number as u64,
			)
			.await?;

			if self.stop {
				return Err(BizError::Bridger("Force stop".to_string()).into());
			}
		}
	}

	/// stop
	pub fn stop(&mut self) {
		info!("💤 SERVICE STOPPED: SUBSCRIBE");
		self.stop = true;
	}

	async fn handle_delayed_extrinsics(
		&mut self,
		header: &<DarwiniaRuntime as System>::Header,
	) -> Result<()> {
		let mut to_removes = vec![];
		for (delayed_to, delayed_ex) in self.delayed_extrinsics.iter() {
			if header.number >= *delayed_to {
				if self
					.darwinia
					.sender
					.need_to_sign_mmr_root_of(*delayed_to)
					.await
				{
					let msg = MsgExtrinsic(delayed_ex.clone());
					self.extrinsics_service.send(msg).await?;
				}
				to_removes.push(*delayed_to);
			}
		}
		for to_remove in to_removes {
			self.delayed_extrinsics.remove(&to_remove);
		}
		Ok(())
	}

	async fn handle_events(
		&mut self,
		header: &<DarwiniaRuntime as System>::Header,
		events: Result<Vec<RawEvent>>,
	) -> Result<()> {
		for event in events? {
			let module = event.module.as_str();
			let variant = event.variant.as_str();
			let event_data = event.data;

			self.handle_event(header, module, variant, event_data)
				.await?;
		}
		Ok(())
	}

	async fn handle_event(
		&mut self,
		_header: &<DarwiniaRuntime as System>::Header,
		module: &str,
		variant: &str,
		event_data: Vec<u8>,
	) -> Result<()> {
		if module != "System" {
			trace!(">> Event - {}::{}", module, variant);
		}

		match (module, variant) {
			("System", "CodeUpdated") => {
				return Err(Error::RuntimeUpdated.into());
			}

			// call ethereum_relay_authorities.request_authority and then sudo call
			// EthereumRelayAuthorities.add_authority will emit the event
			("EthereumRelayAuthorities", "ScheduleAuthoritiesChange") => {
				if self.darwinia.sender.is_authority().await? {
					if let Ok(decoded) =
						ScheduleAuthoritiesChange::<DarwiniaRuntime>::decode(&mut &event_data[..])
					{
						info!(">> Event - {}::{:#?}", module, decoded);
						if self
							.darwinia
							.sender
							.need_to_sign_authorities(decoded.message)
							.await?
						{
							let ex = Extrinsic::SignAndSendAuthorities(decoded.message);
							let msg = MsgExtrinsic(ex);
							self.extrinsics_service.send(msg).await?;
						}
					}
				}
			}

			// authority set changed will emit this event
			("EthereumRelayAuthorities", "AuthoritiesChangeSigned") => {
				if let Ok(decoded) =
					AuthoritiesChangeSigned::<DarwiniaRuntime>::decode(&mut &event_data[..])
				{
					// TODO: Add better repeating check
					info!(">> Event - {}::{:#?}", module, decoded);
					let current_term = self.darwinia.get_current_authority_term().await?;
					if decoded.term == current_term {
						let message = Darwinia::construct_authorities_message(
							self.spec_name.clone(),
							decoded.term,
							decoded.new_authorities,
						);
						let signatures = decoded
							.signatures
							.iter()
							.map(|s| s.1.clone())
							.collect::<Vec<_>>();
						self.ethereum
							.submit_authorities_set(message, signatures)
							.await?;
						info!("Authorities submitted to ethereum");
					}
				}
			}

			// call ethereum_backing.lock will emit the event
			("EthereumRelayAuthorities", "ScheduleMMRRoot") => {
				if self.darwinia.sender.is_authority().await? {
					if let Ok(decoded) =
						ScheduleMMRRoot::<DarwiniaRuntime>::decode(&mut &event_data[..])
					{
						info!(">> Event - {}::{:#?}", module, decoded);
						let ex = Extrinsic::SignAndSendMmrRoot(decoded.block_number);
						self.delayed_extrinsics.insert(decoded.block_number, ex);
					}
				}
			}

			_ => {}
		}

		Ok(())
	}
}
