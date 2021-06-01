#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

#[cfg(feature = "millau")]
pub use declaration::millau::*;
#[cfg(feature = "pangolin")]
pub use declaration::pangolin::*;
pub use types::inner::*;

mod declaration;
mod macros;
pub mod s2s;
pub mod types;
