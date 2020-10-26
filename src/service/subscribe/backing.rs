//! Ethereum Backing Events Handler
use crate::result::Result;
use primitives::{
    frame::ethereum::backing::{RedeemDeposit, RedeemKton, RedeemRing},
    runtime::DarwiniaRuntime,
};
use substrate_subxt::{sp_core::Decode, RawEvent};

// Attributes
const REDEEM_DEPOSIT: &str = "redeem_deposit";
const REDEEM_KTON: &str = "redeem_kton";
const REDEEM_RING: &str = "redeem_ring";

/// Handle the Ethereum Backing Events
pub fn handle(event: RawEvent) -> Result<()> {
    match event.variant.as_str() {
        REDEEM_RING => redeem_ring(event),
        REDEEM_KTON => redeem_kton(event),
        REDEEM_DEPOSIT => redeem_deposit(event),
        _ => {}
    };
    Ok(())
}

fn redeem_ring(event: RawEvent) {
    if let Ok(res) = RedeemRing::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn redeem_kton(event: RawEvent) {
    if let Ok(res) = RedeemKton::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}
fn redeem_deposit(event: RawEvent) {
    if let Ok(res) = RedeemDeposit::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}
