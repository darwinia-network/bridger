//! Guard Service
use std::{
    sync::Arc
};
use actix::prelude::*;

use crate::{
    api::Darwinia,
    error::Result,
};
use crate::service::MsgStop;
use crate::error::Error;

/// MsgSign
#[derive(Clone, Debug)]
pub struct MsgSign {
    /// message
    pub message: Vec<u8>,
}

impl Message for MsgSign {
    type Result = ();
}

/// Sign Service
pub struct SignService {
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
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

impl Handler<MsgSign> for SignService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: MsgSign, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(|_, this, _| {
                    let f = SignService::sign(this.darwinia.clone(), msg.message);
                    f.into_actor(this)
                })
                .map(|r, _, _| {
                    if let Err(err) = r {
                        if let Error::BizError(..) = err {
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
    pub fn new(darwinia: Arc<Darwinia>, is_authority: bool) -> Option<SignService> {
        if is_authority {
            Some(SignService {
                darwinia,
            })
        } else {
            warn!("    🙌 SIGN SERVICE NOT STARTED, YOU ARE NOT AUTHORITY");
            None
        }
    }

    async fn sign(darwinia: Arc<Darwinia>, message: Vec<u8>) -> Result<()> {
        trace!("Sign and sending ...");
        darwinia.sign_submit_signed_authorities(&message).await?;
        Ok(())
    }
}
