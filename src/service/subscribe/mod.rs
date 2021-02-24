//! Darwinia Subscribe
mod darwinia_tracker;

use crate::api::Ethereum;
use crate::error::{BizError, Error};
use crate::service::subscribe::darwinia_tracker::DarwiniaBlockTracker;
use crate::tools;
use crate::{
	error::Result,
	service::extrinsics::{Extrinsic, MsgExtrinsic},
};
use actix::Recipient;
use primitives::runtime::DarwiniaRuntime;
use std::collections::HashMap;
use std::path::PathBuf;
use substrate_subxt::system::System;
use tokio::time::{delay_for, Duration};

use darwinia::{Darwinia2Ethereum, EventInfo, ToEthereumAccount};

/// Dawrinia Subscribe
pub struct SubscribeService {
	darwinia2ethereum: Darwinia2Ethereum,
	account: ToEthereumAccount,
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
		darwinia2ethereum: Darwinia2Ethereum,
		account: ToEthereumAccount,
		ethereum: Ethereum,
		extrinsics_service: Recipient<MsgExtrinsic>,
		spec_name: String,
		scan_from: u32,
		data_dir: PathBuf,
	) -> SubscribeService {
		SubscribeService {
			darwinia2ethereum,
			account,
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
		let mut tracker =
			DarwiniaBlockTracker::new(self.darwinia2ethereum.darwinia.clone(), self.scan_from);
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
			let events = self
				.darwinia2ethereum
				.darwinia
				.get_events_from_block_hash(hash)
				.await;
			if let Err(err) = self.handle_events(&header, events).await {
				if let Some(Error::RuntimeUpdated) = err.downcast_ref() {
					tools::set_cache(
						self.data_dir.clone(),
						tools::LAST_TRACKED_DARWINIA_BLOCK_FILE_NAME,
						header.number as u64,
					)
					.await?;
					return Err(err);
				} else if let Some(jsonrpsee::client::RequestError::Timeout) = err.downcast_ref() {
					tools::set_cache(
						self.data_dir.clone(),
						tools::LAST_TRACKED_DARWINIA_BLOCK_FILE_NAME,
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
				tools::LAST_TRACKED_DARWINIA_BLOCK_FILE_NAME,
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
		let cloned = self.delayed_extrinsics.clone();
		for (delayed_to, delayed_ex) in cloned.iter() {
			if header.number >= *delayed_to
				&& self
					.darwinia2ethereum
					.need_to_sign_mmr_root_of(&self.account, *delayed_to, Some(header.number))
					.await?
			{
				let msg = MsgExtrinsic(delayed_ex.clone());
				self.extrinsics_service.send(msg).await?;
				self.delayed_extrinsics.remove(&delayed_to);
			}
		}
		Ok(())
	}

	async fn handle_events(
		&mut self,
		header: &<DarwiniaRuntime as System>::Header,
		events: Result<Vec<EventInfo<DarwiniaRuntime>>>,
	) -> Result<()> {
		for event in events? {
			self.handle_event(header, event).await?;
		}
		Ok(())
	}

	async fn handle_event(
		&mut self,
		header: &<DarwiniaRuntime as System>::Header,
		event: EventInfo<DarwiniaRuntime>,
	) -> Result<()> {
		//todo
		//if module != "System" {
		//trace!(">> Event - {}::{}", module, variant);
		//}
		let block = Some(header.number);
		match event {
			EventInfo::RuntimeUpdatedEvent(_) => {
				return Err(Error::RuntimeUpdated.into());
			}
			// call ethereum_relay_authorities.request_authority and then sudo call
			// EthereumRelayAuthorities.add_authority will emit the event
			EventInfo::ScheduleAuthoritiesChangeEvent(event) => {
				if self
					.darwinia2ethereum
					.is_authority(block, &self.account)
					.await? && self
					.darwinia2ethereum
					.need_to_sign_authorities(block, &self.account, event.message)
					.await?
				{
					let ex = Extrinsic::SignAndSendAuthorities(event.message);
					let msg = MsgExtrinsic(ex);
					self.extrinsics_service.send(msg).await?;
				}
			}
			// authority set changed will emit this event
			EventInfo::AuthoritiesChangeSignedEvent(event) => {
				let current_term = self.darwinia2ethereum.get_current_authority_term().await?;
				if event.term == current_term {
					let message = Darwinia2Ethereum::construct_authorities_message(
						self.spec_name.clone(),
						event.term,
						event.new_authorities,
					);
					let signatures = event
						.signatures
						.iter()
						.map(|s| s.1.clone())
						.collect::<Vec<_>>();
					let tx_hash = self
						.ethereum
						.submit_authorities_set(message, signatures)
						.await?;
					info!("Submit authorities to ethereum with tx: {}", tx_hash);
				}
			}
			// call ethereum_backing.lock will emit the event
			EventInfo::ScheduleMMRRootEvent(event) => {
				if self
					.darwinia2ethereum
					.is_authority(block, &self.account)
					.await?
				{
					info!("{}", event);
					let ex = Extrinsic::SignAndSendMmrRoot(event.block_number);
					self.delayed_extrinsics.insert(event.block_number, ex);
				}
			}
			_ => {}
		}
		Ok(())
	}
}
