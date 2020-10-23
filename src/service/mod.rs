//! Bridger Services
use crate::{pool::Pool, result::Result};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

mod ethereum;
mod guard;
mod redeem;
mod relay;
mod subscribe;

pub use self::{
    ethereum::EthereumService, guard::GuardService, redeem::RedeemService, relay::RelayService,
};

/// Bridge service
#[async_trait(?Send)]
pub trait Service {
    /// Service name
    fn name<'c>(&self) -> &'c str;
    /// Run target service
    async fn run(&mut self, pool: Arc<Mutex<Pool>>) -> Result<()>;
}
