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
use std::{
    sync::Arc
};
use substrate_subxt::sp_core::Decode;
use jsonrpsee::client::Subscription;
use substrate_subxt::RawEvent;
use substrate_subxt::system::System;
use std::collections::HashMap;

/// Dawrinia Subscribe
pub struct SubscribeService {
    darwinia: Arc<Darwinia>,
	ethereum: Ethereum,
	stop: bool,
	extrinsics_service: Recipient<MsgExtrinsic>,
    delayed_extrinsics: HashMap<u32, Extrinsic>,
}

impl SubscribeService {
	/// New subscribe service
	pub fn new(
		darwinia: Arc<Darwinia>,
		ethereum: Ethereum,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> SubscribeService {
		SubscribeService {
            darwinia,
			ethereum,
			stop: false,
            extrinsics_service,
            delayed_extrinsics: HashMap::new(),
		}
	}

    /// start
    pub async fn start(&mut self) -> Result<()> {
        let mut sub: Subscription<<DarwiniaRuntime as System>::Header> = self.darwinia.client.subscribe_finalized_blocks().await?;
        info!("✨ SERVICE STARTED: SUBSCRIBE");
        loop {
            let header = sub.next().await;
            let hash = header.hash();
            trace!("Block {}", header.number);

            if let Err(err) = self.handle_delayed_extrinsics(&header).await {
                error!("Encounter error when handle delayed extrinsics: {:#?}", err);
            }

            // events
            let events = self.darwinia.get_raw_events(hash).await;
            if let Err(err) = self.handle_events(&header, events).await {
                error!("Encounter error when handle events of block {}: {:#?}", header.number, err);
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

    async fn handle_delayed_extrinsics(&mut self, header: &<DarwiniaRuntime as System>::Header) -> Result<()> {
        let mut to_removes = vec![];
        for (delayed_to, delayed_ex) in self.delayed_extrinsics.iter() {
            if header.number >= *delayed_to {
                let msg = MsgExtrinsic(delayed_ex.clone());
                self.extrinsics_service.send(msg).await?;
                to_removes.push(delayed_to.clone());
            }
        }
        for to_remove in to_removes {
            self.delayed_extrinsics.remove(&to_remove);
        }
        Ok(())
    }

    async fn handle_events(&mut self, header: &<DarwiniaRuntime as System>::Header, events: Result<Vec<RawEvent>>) -> Result<()> {
        match events {
            Ok(events) => {
                for event in events {
                    let module = event.module.as_str();
                    let variant = event.variant.as_str();
                    let event_data = event.data;

                    self.handle_event(header, module, variant, event_data).await?;
                }
            },
            Err(err) => {
                error!("{:#?}", err);
            }
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
                            self.delayed_extrinsics.insert(decoded.block_number, ex);
                    }
			    }
            }

			_ => {}
		}

		Ok(())
	}
}
