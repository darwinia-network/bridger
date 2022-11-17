pub use support_types::mark::BridgeName;

#[cfg(feature = "bridge-ethv2")]
pub use self::bridge_ethv2::*;
#[cfg(feature = "bridge-s2s")]
pub use self::bridge_s2s::*;
pub use self::subquery::*;

#[cfg(feature = "bridge-ethv2")]
mod bridge_ethv2;
#[cfg(feature = "bridge-s2s")]
mod bridge_s2s;
mod subquery;
