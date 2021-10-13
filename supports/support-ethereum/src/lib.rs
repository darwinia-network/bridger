#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::manual_range_contains)]

#[macro_use]
extern crate bridge_primitives;

pub mod affirmation;
pub mod block;
pub mod ethash;
pub mod mmr;
pub mod parcel;
pub mod proof;
pub mod proxy_type;
pub mod receipt;
