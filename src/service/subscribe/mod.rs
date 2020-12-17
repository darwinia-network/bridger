//! Darwinia Subscribe
use crate::api::Ethereum;
use crate::error::BizError;
use crate::{
	api::Darwinia,
	error::Result,
    service::extrinsics::{Extrinsic, MsgExtrinsic},
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
    darwinia: Arc<Darwinia>,
	sub: EventSubscription<DarwiniaRuntime>,
	ethereum: Ethereum,
	stop: bool,
	extrinsics_service: Recipient<MsgExtrinsic>,
}

impl SubscribeService {
	/// New redeem service
	pub async fn new(
		darwinia: Arc<Darwinia>,
		ethereum: Ethereum,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Result<SubscribeService> {
        let sub = darwinia.build_event_subscription().await?;
		Ok(SubscribeService {
            darwinia,
			sub,
			ethereum,
			stop: false,
            extrinsics_service,
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
					error!("{:#?}", e);
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
            match raw {
                Ok(event) => {
                    self.handle_event(&event.module, &event.variant, event.data)
                        .await?;
                },
                Err(err) => {
                    return Err(err.into()); 
                }
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

            // call ethereum_relay_authorities.request_authority and then sudo call
            // EthereumRelayAuthorities.add_authority will emit the event
			("EthereumRelayAuthorities", "NewAuthorities") => {
                if self.darwinia.sender.is_authority().await? {
                    if let Ok(decoded) =
                        NewAuthorities::<DarwiniaRuntime>::decode(&mut &event_data[..])
                    {
                        let ex = Extrinsic::SignAndSendAuthorities(decoded.message);
                        let msg = MsgExtrinsic(ex);
                        self.extrinsics_service.send(msg).await?;
                    }
                }
			}

            // authority set changed will emit this event
			("EthereumRelayAuthorities", "AuthoritiesSetSigned") => {
				if let Ok(decoded) =
					AuthoritiesSetSigned::<DarwiniaRuntime>::decode(&mut &event_data[..])
				{
					self.ethereum.submit_authorities_set(
                        decoded.new_authorities,
						decoded.signatures,
					).await?;
                    info!("Authorities submitted to ethereum");
				}
			}

            // call ethereum_backing.lock will emit the event
			("EthereumRelayAuthorities", "NewMMRRoot") => {
                if self.darwinia.sender.is_authority().await? {
                    if let Ok(decoded) = NewMMRRoot::<DarwiniaRuntime>::decode(&mut &event_data[..])
                    {
                        let ex = Extrinsic::SignAndSendMmrRoot(decoded.block_number);
                        let msg = MsgExtrinsic(ex);
                        self.extrinsics_service.send(msg).await?;
                    }
			    }
            }

			_ => {}
		}

		Ok(())
	}
}
