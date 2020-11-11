//! Ethereum Relayer Game Events Handler
use crate::result::Result;
use primitives::{
    frame::ethereum::relay::{
        Affirmed, DisputedAndAffirmed, Extended, GameOver, NewRound, Pended,
        PendingRelayHeaderParcelApproved, PendingRelayHeaderParcelRejected, RemoveConfirmedParcel,
        VerifyReceipt,
    },
    runtime::DarwiniaRuntime,
};
use substrate_subxt::{sp_core::Decode, RawEvent};

// Attributes
const AFFIRMED: &str = "Affirmed";
const DISPUTED_AND_AFFIRMED: &str = "DisputedAndAffirmed";
const EXTENDED: &str = "Extended";
const NEW_ROUND: &str = "NewRound";
const GAME_OVER: &str = "GameOver";
const REMOVE_CONFIRMED_PARCEL: &str = "RemoveConfirmedParcel";
const VERIFY_RECEIPT: &str = "VerifyReceipt";
const PENDED: &str = "Pended";
const PENDING_RELAY_HEADER_PARCEL_APPROVED: &str = "PendingRelayHeaderParcelApproved";
const PENDING_RELAY_HEADER_PARCEL_REJECTED: &str = "PendingRelayHeaderParcelRejected";

/// Handle the Ethereum Relayer Game Events
pub fn handle(event: RawEvent) -> Result<()> {
    match event.variant.as_str() {
        AFFIRMED => affirmed(event),
        DISPUTED_AND_AFFIRMED => disputed_and_affirmed(event),
        EXTENDED => extended(event),
        GAME_OVER => game_over(event),
        REMOVE_CONFIRMED_PARCEL => remove_confirmed_parcel(event),
        VERIFY_RECEIPT => verify_receipt(event),
        PENDED => pended(event),
        NEW_ROUND => new_round(event),
        PENDING_RELAY_HEADER_PARCEL_APPROVED => pending_relay_header_parcel_approved(event),
        PENDING_RELAY_HEADER_PARCEL_REJECTED => pending_relay_header_parcel_rejected(event),
        _ => {}
    };
    Ok(())
}

fn affirmed(event: RawEvent) {
    if let Ok(res) = Affirmed::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn disputed_and_affirmed(event: RawEvent) {
    if let Ok(res) = DisputedAndAffirmed::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}
fn extended(event: RawEvent) {
    if let Ok(res) = Extended::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn new_round(event: RawEvent) {
    if let Ok(res) = NewRound::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn game_over(event: RawEvent) {
    if let Ok(res) = GameOver::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn pended(event: RawEvent) {
    if let Ok(res) = Pended::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn pending_relay_header_parcel_approved(event: RawEvent) {
    if let Ok(res) =
        PendingRelayHeaderParcelApproved::<DarwiniaRuntime>::decode(&mut &event.data[..])
    {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn pending_relay_header_parcel_rejected(event: RawEvent) {
    if let Ok(res) =
        PendingRelayHeaderParcelRejected::<DarwiniaRuntime>::decode(&mut &event.data[..])
    {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn remove_confirmed_parcel(event: RawEvent) {
    if let Ok(res) = RemoveConfirmedParcel::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}

fn verify_receipt(event: RawEvent) {
    if let Ok(res) = VerifyReceipt::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        trace!(
            ">> Event - {}::{}\n{:#?}",
            event.module,
            event.variant,
            res
        );
    }
}
