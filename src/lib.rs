//! ## Darwinia bridge primitives
//!
//! [![bridge-primtives](https://github.com/darwinia-network/bridge-primitives/workflows/bridge-primtives/badge.svg)](https://github.com/darwinia-network/bridge-primitives)
//! [![crate](https://img.shields.io/crates/v/darwinia-bridge-primitives.svg)](https://crates.io/crates/darwinia-bridge-pritmives)
//! [![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia-bridge-pritmives/)
//! [![downloads](https://img.shields.io/crates/d/darwinia-bridge-primitives.svg)](https://crates.io/crates/darwinia-bridge-primitives)
//! [![LICENSE](https://img.shields.io/crates/l/darwinia-bridge-primitives.svg)](https://choosealicense.com/licenses/gpl-3.0/)
//!
//! The Darwinia bridge primtives
//!
//!
//! + [x] [Ethereum](https://github.com/darwinia-network/bridge-primitives/tree/master/src/eth)
//! + [ ] [Your chain?](https://github.com/darwinia-network/bridge-primitives/pulls)
//!
//! ## Features
//!
//! ### `rpc`
//!
//! If you want to trigger rpc feature to get data which described in this repo, please
//! import `darwinia-bridge-primitives` as below in your `Cargo.toml`
//!
//! ```toml
//! [dependencies.darwinia-bridge-primitives]
//! version = "^0"
//! features = [ "rpc" ]
//! ```
//!
//!
//! ## LICENSE
//!
//! GPL-3.0
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::assign_op_pattern)]
#![warn(missing_docs)]
#[macro_use]
extern crate serde;

// macros
mod byte;

pub mod array;
pub mod chain;
pub mod frame;
pub mod result;
pub mod rpc;
pub mod runtime;
