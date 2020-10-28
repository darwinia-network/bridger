//! Bridger Listener
use crate::{
    memcache::MemCache,
    result::Result,
    service::Service,
};
use std::sync::{Arc, Mutex};

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
    pub async fn start(&mut self, start: u64) -> Result<()> {
        let pool = Arc::new(Mutex::new(MemCache::new(start)));
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
}
