//! Darwinia Subscribe
use crate::api::Ethereum;
use crate::error::BizError;
use crate::{
	api::Darwinia,
	error::Result,
	service::sign::{MsgToSignAuthorities, MsgToSignMMRRoot},
};
use actix::Recipient;
use primitives::{
	frame::bridge::relay_authorities::{AuthoritiesSetSigned, NewAuthorities, NewMMRRoot},
	runtime::DarwiniaRuntime,
};
use std::sync::Arc;
use substrate_subxt::sp_core::Decode;
use substrate_subxt::EventSubscription;

/// Dawrinia Subscribe
pub struct SubscribeService {
	sub: EventSubscription<DarwiniaRuntime>,
	ethereum: Ethereum,
	stop: bool,
	sign_authorities: Option<Recipient<MsgToSignAuthorities>>,
	sign_mmr_root: Option<Recipient<MsgToSignMMRRoot>>,
}

impl SubscribeService {
	/// New redeem service
	pub async fn new(
		darwinia: Arc<Darwinia>,
		ethereum: Ethereum,
		sign_authorities: Option<Recipient<MsgToSignAuthorities>>,
		sign_mmr_root: Option<Recipient<MsgToSignMMRRoot>>,
	) -> Result<SubscribeService> {
		Ok(SubscribeService {
			sub: darwinia.build_event_subscription().await?,
			ethereum,
			stop: false,
			sign_authorities,
			sign_mmr_root,
		})
	}

	/// start
	pub async fn start(&mut self) -> Result<SubscribeService> {
		info!("✨ SERVICE STARTED: SUBSCRIBE");
		loop {
			if let Err(e) = self.process_next_event().await {
				if e.to_string() == "CodeUpdated" {
					self.stop();
					return Err(e);
				} else {
					error!("Fail to process next event: {:?}", e);
				}
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

	/// process_next_event
	async fn process_next_event(&mut self) -> Result<()> {
		if let Some(raw) = self.sub.next().await {
			if let Ok(event) = raw {
				self.handle_event(&event.module, &event.variant, event.data)
					.await?;
			}
		}
		Ok(())
	}

	async fn handle_event(
		&mut self,
		module: &str,
		variant: &str,
		event_data: Vec<u8>,
	) -> Result<()> {
		if module != "System" {
			debug!(">> Event - {}::{}", module, variant);
		}

		match (module, variant) {
			("System", "CodeUpdated") => {
				return Err(BizError::Bridger("CodeUpdated".to_string()).into());
			}

			("EthereumRelayAuthorities", "NewAuthorities") => {
				if let Some(sign_authorities) = &self.sign_authorities {
					if let Ok(decoded) =
						NewAuthorities::<DarwiniaRuntime>::decode(&mut &event_data[..])
					{
						let msg = MsgToSignAuthorities(decoded.message);
						sign_authorities.send(msg).await?;
					}
				}
			}

			("EthereumRelayAuthorities", "AuthoritiesSetSigned") => {
				if let Ok(decoded) =
					AuthoritiesSetSigned::<DarwiniaRuntime>::decode(&mut &event_data[..])
				{
					self.ethereum.submit_authorities_set(
						decoded.term,
						decoded.message,
						decoded.signatures,
					).await?;
				}
			}

			("EthereumRelayAuthorities", "NewMMRRoot") => {
				if let Some(sign_mmr_root) = &self.sign_mmr_root {
					if let Ok(decoded) = NewMMRRoot::<DarwiniaRuntime>::decode(&mut &event_data[..])
					{
						let msg = MsgToSignMMRRoot(decoded.block_number);
						sign_mmr_root.send(msg).await?;
					}
				}
			}

			_ => {}
		}

		Ok(())
	}
}
