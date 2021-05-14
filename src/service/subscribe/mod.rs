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
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::time::{delay_for, Duration};

use darwinia::{Darwinia2Ethereum, EventInfo, ToEthereumAccount};

use substrate_subxt::{
    Runtime,
    system::System,
    balances::Balances,
	sp_runtime::{
		generic::Header,
		traits::{
            BlakeTwo256,
            Verify,
        },
	},
    sp_core::H256,
};

use primitives::frame::bridge::relay_authorities::EthereumRelayAuthorities;

use primitives::{
	chain::{
        ethereum::{
            EcdsaAddress,
            EcdsaSignature,
            EcdsaMessage,
        },
    },
	frame::bridge::relay_authorities::RelayAuthority,
};



/// Dawrinia Subscribe
pub struct SubscribeService<R: Runtime> {
	darwinia2ethereum: Darwinia2Ethereum<R>,
	account: ToEthereumAccount<R>,
	ethereum: Ethereum,
	stop: bool,
	extrinsics_service: Recipient<MsgExtrinsic>,
	delayed_extrinsics: HashMap<u32, Extrinsic>,
	spec_name: String,
	scan_from: u32,
	data_dir: PathBuf,
}

impl<R: Runtime + EthereumRelayAuthorities + Balances> SubscribeService<R> {
	/// New subscribe service
	pub fn new(
		darwinia2ethereum: Darwinia2Ethereum<R>,
		account: ToEthereumAccount<R>,
		ethereum: Ethereum,
		extrinsics_service: Recipient<MsgExtrinsic>,
		spec_name: String,
		scan_from: u32,
		data_dir: PathBuf,
	) -> Self {
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
	pub async fn start(&mut self) -> Result<()> 
        where R: System<BlockNumber = u32, Header = Header<u32, BlakeTwo256>, Hash = H256>,
              <R as Runtime>::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
              <<R as Runtime>::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
              R: EthereumRelayAuthorities<
                  RelayAuthority = RelayAuthority<
                  <R as System>::AccountId,
                  EcdsaAddress,
                  <R as Balances>::Balance,
                  <R as System>::BlockNumber,
                  >,
                RelayAuthorityMessage = EcdsaMessage,
                RelayAuthoritySigner = EcdsaAddress,
                RelayAuthoritySignature = EcdsaSignature,
              >,
    {
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
				.await
				.map_err(|err| err.into());
			if let Err(err) = self.handle_events(&header, events).await {
				if let Some(Error::RuntimeUpdated) = err.downcast_ref() {
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
					delay_for(Duration::from_secs(30)).await;
				}
			} else {
				tools::set_cache(
					self.data_dir.clone(),
					tools::LAST_TRACKED_DARWINIA_BLOCK_FILE_NAME,
					header.number as u64,
				)
				.await?;
			}

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
		header: &Header<u32, BlakeTwo256>,
	) -> Result<()> 
        where R: System<BlockNumber = u32>
    {
		let cloned = self.delayed_extrinsics.clone();
		for (delayed_to, delayed_ex) in cloned.iter() {
			if header.number >= *delayed_to
				&& self
					.darwinia2ethereum
					.need_to_sign_mmr_root_of(&self.account, *delayed_to, Some(header.number))
					.await?
			{
				tools::send_extrinsic(&self.extrinsics_service, delayed_ex.clone()).await;
				self.delayed_extrinsics.remove(&delayed_to);
			}
		}
		Ok(())
	}

	async fn handle_events(
		&mut self,
		header: &Header<u32, BlakeTwo256>,
		events: Result<Vec<EventInfo<R>>>,
	) -> Result<()> 
        where R: EthereumRelayAuthorities<
				RelayAuthority = RelayAuthority<
					<R as System>::AccountId,
					EcdsaAddress,
					<R as Balances>::Balance,
					<R as System>::BlockNumber,
				>,
                RelayAuthorityMessage = EcdsaMessage,
                RelayAuthoritySigner = EcdsaAddress,
                RelayAuthoritySignature = EcdsaSignature,
			>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
		<R::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
        R: System<BlockNumber = u32>
    {
		for event in events? {
			self.handle_event(header, event).await?;
		}
		Ok(())
	}

	async fn handle_event(
		&mut self,
		header: &Header<u32, BlakeTwo256>,
		event: EventInfo<R>,
	) -> Result<()> 
        where R: EthereumRelayAuthorities<
				RelayAuthority = RelayAuthority<
					<R as System>::AccountId,
					EcdsaAddress,
					<R as Balances>::Balance,
					<R as System>::BlockNumber,
				>,
                RelayAuthorityMessage = EcdsaMessage,
                RelayAuthoritySigner = EcdsaAddress,
                RelayAuthoritySignature = EcdsaSignature,
			>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
		<R::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
        R: System<BlockNumber = u32>,
    {
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
					tools::send_extrinsic(&self.extrinsics_service, ex).await;
				}
			}
			// authority set changed will emit this event
			EventInfo::AuthoritiesChangeSignedEvent(event) => {
				let current_term = self.darwinia2ethereum.get_current_authority_term().await?;
				if event.term == current_term {
					let message = Darwinia2Ethereum::<R>::construct_authorities_message(
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
