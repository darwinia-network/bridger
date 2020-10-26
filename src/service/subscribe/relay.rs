//! Ethereum Relay Service Events handler
use crate::result::Result;
use primitives::{
    frame::ethereum::relay::{RemoveConfirmedParcel, VerifyReceipt},
    runtime::DarwiniaRuntime,
};
use substrate_subxt::{sp_core::Decode, RawEvent};

// Attributes
const REMOVE_CONFIRMED_PARCEL: &str = "remove_confirmed_parcel";
const VERIFY_RECEIPT: &str = "remove_receipt";

/// Ethereum Relay Event Handler
pub fn handle(event: RawEvent) -> Result<()> {
    match event.variant.as_str() {
        REMOVE_CONFIRMED_PARCEL => remove_confirmed_parcel(event),
        VERIFY_RECEIPT => verify_receipt(event),
        _ => {}
    }
    Ok(())
}

fn remove_confirmed_parcel(event: RawEvent) {
    if let Ok(res) = RemoveConfirmedParcel::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        println!("{:?}", res);
    }
}

fn verify_receipt(event: RawEvent) {
    if let Ok(res) = VerifyReceipt::<DarwiniaRuntime>::decode(&mut &event.data[..]) {
        println!("{:?}", res);
    }
}
