//! Darwinia bridger
#![warn(missing_docs)]

#[macro_use]
extern crate log;

mod config;

pub mod api;
pub mod cmd;
pub mod result;
pub mod service;
pub mod memcache;

pub use self::{config::Config, service::Service};
