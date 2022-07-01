#[cfg(feature = "bridge-parachain")]
pub use self::bridge_parachain::*;
pub use self::bridge_solochain::*;

#[cfg(feature = "bridge-parachain")]
mod bridge_parachain;
mod bridge_solochain;
mod common;
