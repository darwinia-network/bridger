//! Bridger Services
pub use crate::error::Result;

pub mod ethereum;
pub mod guard;
pub mod redeem;
pub mod relay;
pub mod subscribe;
pub mod sign;

pub use self::{
    ethereum::EthereumService,
    redeem::RedeemService,
    relay::RelayService,
    guard::GuardService,
    subscribe::SubscribeService,
    sign::SignService,
};
use actix::Message;

/// Msg to stop service
#[derive(Clone, Debug)]
pub struct MsgStop;

impl Message for MsgStop {
    type Result = ();
}

