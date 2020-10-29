//! Ethereum Backing Events Handler
use crate::result::Result;
use primitives::{
    frame::ethereum::backing::{RedeemDeposit, RedeemKton, RedeemRing},
    runtime::DarwiniaRuntime,
};
use substrate_subxt::{sp_core::Decode, RawEvent};

// Attributes
const REDEEM_DEPOSIT: &str = "RedeemDeposit";
const REDEEM_KTON: &str = "RedeemKton";
const REDEEM_RING: &str = "RedeemRing";

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
        trace!(
            ">> Event - {}::{}\n\t{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn redeem_kton(event: RawEvent) {
    if let Ok(res) = RedeemKton::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n\t{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}
fn redeem_deposit(event: RawEvent) {
    if let Ok(res) = RedeemDeposit::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n\t{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}
