//! Bridger Listener
use crate::service::Service;

/// Bridger listener
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
}
