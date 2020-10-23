//! Bridger Listener
use crate::{
    api::{Darwinia, Shadow},
    pool::Pool,
    result::{Error, Result},
    service::{EthereumService, GuardService, RedeemService, RelayService, Service},
    Config,
};
use std::sync::{Arc, Mutex};
use web3::transports::http::Http;

/// Bridger listener
#[derive(Default)]
pub struct Listener(Vec<Box<dyn Service>>);

impl Listener {
    /// Get service
    pub fn entry(&self, name: &str) -> Option<&dyn Service> {
        for s in &self.0 {
            if s.name() == name {
                return Some(s.as_ref());
            }
        }
        None
    }

    /// Register service
    pub fn register<S: Service + 'static>(&mut self, service: S) -> Result<()>
    where
        S: Service,
    {
        self.0.push(Box::new(service));
        Ok(())
    }

    /// Start services
    pub async fn start(&mut self) -> Result<()> {
        let pool = Arc::new(Mutex::new(Pool::default()));
        let result = futures::future::join_all(self.0.iter_mut().map(|s| {
            info!("Start service {}", s.name());
            s.run(Arc::clone(&pool))
        }))
        .await;
        for r in result {
            r?;
        }
        Ok(())
    }

    /// Generate listener from `Config`
    pub async fn from_config(config: Config) -> Result<Self> {
        let mut l = Self::default();
        if config.eth.rpc.starts_with("ws") {
            return Err(Error::Bridger(
                "Bridger currently doesn't support ethereum websocket transport".to_string(),
            ));
        }

        // APIs
        let shadow = Arc::new(Shadow::new(&config));
        let darwinia = Arc::new(Darwinia::new(&config).await?);

        // 1. Transaction Listener
        // 2. Relay Listener
        let ethereum = <EthereumService<Http>>::new_http(&config)?;
        let relay = RelayService::new(&config, shadow.clone(), darwinia.clone());
        let redeem = RedeemService::new(&config, shadow.clone(), darwinia.clone());
        let guard = GuardService::new(&config, shadow, darwinia);

        // Register
        l.register(ethereum)?;
        l.register(relay)?;
        l.register(redeem)?;
        l.register(guard)?;
        Ok(l)
    }
}
