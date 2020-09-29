//! Bridger services
use crate::{result::Result, Config};
use async_trait::async_trait;

/// Bridge service
#[async_trait]
pub trait Service {
    /// Service name
    fn name<'c>(&self) -> &'c str;
    /// Run target service
    async fn run(&self, config: &Config) -> Result<()>;
}
