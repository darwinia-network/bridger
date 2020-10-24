//! Relayable chains
mod affirmation;
pub mod ethereum;
/// proxy type
pub mod proxy_type;

pub use self::affirmation::{RelayAffirmation, RelayAffirmationId};
