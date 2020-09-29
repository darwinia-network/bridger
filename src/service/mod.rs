//! Bridger services
use crate::{pool::Pool, result::Result};
use async_trait::async_trait;
use std::{cell::RefCell, sync::Arc};

mod eth;

pub use eth::EthereumService;

/// Bridge service
#[async_trait(?Send)]
pub trait Service {
    /// Service name
    fn name<'c>(&self) -> &'c str;
    /// Run target service
    async fn run(&mut self, pool: Arc<RefCell<Pool>>) -> Result<()>;
}
