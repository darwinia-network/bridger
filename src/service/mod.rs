//! Bridger services
use crate::{pool::Pool, result::Result};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

mod eth;
mod relay;

pub use self::{eth::EthereumService, relay::RelayService};

/// Bridge service
#[async_trait(?Send)]
pub trait Service {
    /// Service name
    fn name<'c>(&self) -> &'c str;
    /// Run target service
    async fn run(&mut self, pool: Arc<Mutex<Pool>>) -> Result<()>;
}
