//! Bridger Services
pub use crate::error::Result;

pub mod ethereum;
pub mod extrinsics;
pub mod guard;
pub mod redeem;
pub mod relay;
pub mod subscribe;

pub use self::{
	ethereum::EthereumService, extrinsics::ExtrinsicsService, guard::GuardService,
	redeem::RedeemService, relay::RelayService, subscribe::SubscribeService,
};
use actix::Message;

/// Msg to stop service
#[derive(Clone, Debug)]
pub struct MsgStop;

impl Message for MsgStop {
	type Result = ();
}
