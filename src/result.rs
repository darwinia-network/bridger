//! Bridger Result
use etc::Error as Etc;
use reqwest::Error as Reqwest;
use serde_json::Error as SerdeJson;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
    result::Result as StdResult,
};
use substrate_subxt::Error as Subxt;
use toml::{de::Error as DeToml, ser::Error as SerToml};
use web3::Error as Web3;

/// The custom bridger error
pub struct Bridger(String);
impl Display for Bridger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

/// Error generator
macro_rules! error {
    ($($e:ident),*) => {
        /// Bridger Error
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        impl ErrorTrait for Error {}

        $(
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )+
    };
}

error! {Io, Bridger, DeToml, SerToml, Etc, Web3, Reqwest, SerdeJson, Subxt}

/// Bridger Result
pub type Result<T> = StdResult<T, Error>;
