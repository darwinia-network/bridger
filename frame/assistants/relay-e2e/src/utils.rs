use std::str::FromStr;

use web3::types::Address;

use crate::error::{RelayError, RelayResult};

pub fn address_from_str(s: &str) -> RelayResult<Address> {
    Address::from_str(s)
        .map_err(|_| RelayError::Custom(format!("Failed to get address from {}", s)))
}
