//! Bridge Result
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
    result::Result as StdResult,
};

#[cfg(feature = "rpc")]
use reqwest::Error as Reqwest;
#[cfg(feature = "rpc")]
use serde_json::Error as SerdeJson;

/// The custom bridger error
pub struct Bridge(String);
impl Display for Bridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

/// Error generator
macro_rules! error {
    ($($(#[$attr:meta])* $e:ident),*) => {
        /// Bridge Error
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($(#[$attr])* $e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $($(#[$attr])* Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        impl ErrorTrait for Error {}
        $(
            $(#[$attr])*
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )*

    };
}

error! {
    Io,
    Bridge,
    #[cfg(feature = "rpc")]
    Reqwest,
    #[cfg(feature = "rpc")]
    SerdeJson
}

/// Sup Result
pub type Result<T> = StdResult<T, Error>;
