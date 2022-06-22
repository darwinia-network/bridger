#[cfg(feature = "bridge-parachain")]
pub use self::para_head_relay::*;
#[cfg(feature = "bridge-parachain")]
pub use self::relaychain_head_relay::*;
pub use self::solochain_head_relay::*;

#[cfg(feature = "bridge-parachain")]
mod para_head_relay;
#[cfg(feature = "bridge-parachain")]
mod relaychain_head_relay;
mod solochain_head_relay;
