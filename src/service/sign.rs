//! Guard Service
use std::sync::Arc;

use actix::prelude::*;

use crate::{api::Darwinia, error::Result};
use crate::error::BizError;
use crate::service::MsgStop;

/// MsgSign
#[derive(Clone, Debug)]
pub struct MsgToSignAuthorities(pub Vec<u8>);

impl Message for MsgToSignAuthorities {
    type Result = ();
}

/// MsgToSignMMRRoot
#[derive(Clone, Debug)]
pub struct MsgToSignMMRRoot(pub u32);

impl Message for MsgToSignMMRRoot {
    type Result = ();
}

/// Sign Service
pub struct SignService {
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
    /// spec name
    pub spec_name: String,
}

impl Actor for SignService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("     ✨ SERVICE STARTED: SIGN");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("     💤 SERVICE STOPPED: SIGN")
    }
}

impl Handler<MsgToSignAuthorities> for SignService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: MsgToSignAuthorities, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(|_, this, _| {
                    let f = SignService::ecdsa_sign_and_submit_signed_authorities(
                        this.darwinia.clone(),
                        msg.0,
                    );
                    f.into_actor(this)
                })
                .map(|r, _, _| {
                    if let Err(err) = r {
                        if err.downcast_ref::<BizError>().is_some() {
                            trace!("{}", err);
                        } else {
                            error!("{:?}", err);
                        }
                    }
                }),
        ))
    }
}

impl Handler<MsgToSignMMRRoot> for SignService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: MsgToSignMMRRoot, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(move |_, this, _| {
                    let f = SignService::ecdsa_sign_and_submit_signed_mmr_root(
                        this.darwinia.clone(),
                        this.spec_name.clone(),
                        msg.0,
                    );
                    f.into_actor(this)
                })
                .map(|r, _, _| {
                    if let Err(err) = r {
                        if err.downcast_ref::<BizError>().is_some() {
                            trace!("{}", err);
                        } else {
                            error!("{:?}", err);
                        }
                    }
                }),
        ))
    }
}

impl Handler<MsgStop> for SignService {
    type Result = ();

    fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl SignService {
    /// New sign service
    pub fn new(
        darwinia: Arc<Darwinia>,
        is_authority: bool,
        spec_name: String,
    ) -> Option<SignService> {
        if is_authority {
            Some(SignService {
                darwinia,
                spec_name,
            })
        } else {
            warn!("     🙌 SIGN SERVICE NOT STARTED, YOU ARE NOT AUTHORITY");
            None
        }
    }

    async fn ecdsa_sign_and_submit_signed_authorities(
        darwinia: Arc<Darwinia>,
        message: Vec<u8>,
    ) -> Result<()> {
        trace!("Sign and sending authorities...");
        darwinia
            .ecdsa_sign_and_submit_signed_authorities(&message)
            .await?;
        Ok(())
    }

    async fn ecdsa_sign_and_submit_signed_mmr_root(
        darwinia: Arc<Darwinia>,
        spec_name: String,
        block_number: u32,
    ) -> Result<()> {
        trace!("Sign and sending mmr_root...");
        darwinia
            .ecdsa_sign_and_submit_signed_mmr_root(spec_name, block_number)
            .await?;
        Ok(())
    }
}
