//! Ethereum Relayer Game Events Handler
use crate::result::Result;
use primitives::{
    frame::ethereum::game::{
        Affirmed, Disputed, Extended, GameOver, NewRound, PendingRelayHeaderParcelApproved,
        PendingRelayHeaderParcelRejected,
    },
    runtime::DarwiniaRuntime,
};
use substrate_subxt::{sp_core::Decode, RawEvent};

// Attributes
const AFFIRMED: &str = "affirmed";
const DISPUTED: &str = "disputed";
const EXTENDED: &str = "extended";
const GAME_OVER: &str = "game_over";
const NEW_ROUND: &str = "new_round";
const PENDING_RELAY_HEADER_PARCEL_APPROVED: &str = "pending_relay_header_parcel_approved";
const PENDING_RELAY_HEADER_PARCEL_REJECTED: &str = "pending_relay_header_parcel_rejected";

/// Handle the Ethereum Relayer Game Events
pub fn handle(event: RawEvent) -> Result<()> {
    match event.variant.as_str() {
        AFFIRMED => affirmed(event),
        DISPUTED => disputed(event),
        EXTENDED => extended(event),
        GAME_OVER => game_over(event),
        NEW_ROUND => new_round(event),
        PENDING_RELAY_HEADER_PARCEL_APPROVED => pending_relay_header_parcel_approved(event),
        PENDING_RELAY_HEADER_PARCEL_REJECTED => pending_relay_header_parcel_rejected(event),
        _ => {}
    };
    Ok(())
}

fn affirmed(event: RawEvent) {
    if let Ok(res) = Affirmed::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn disputed(event: RawEvent) {
    if let Ok(res) = Disputed::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}
fn extended(event: RawEvent) {
    if let Ok(res) = Extended::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn new_round(event: RawEvent) {
    if let Ok(res) = NewRound::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn game_over(event: RawEvent) {
    if let Ok(res) = GameOver::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn pending_relay_header_parcel_approved(event: RawEvent) {
    if let Ok(res) =
        PendingRelayHeaderParcelApproved::<DarwiniaRuntime>::decode(&mut &event.data[..])
    {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}

fn pending_relay_header_parcel_rejected(event: RawEvent) {
    if let Ok(res) =
        PendingRelayHeaderParcelRejected::<DarwiniaRuntime>::decode(&mut &event.data[..])
    {
        trace!(">> {}::{}\n\t{:?}", event.module, event.variant, res);
    }
}
