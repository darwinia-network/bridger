extern crate proc_macro;

use proc_macro::TokenStream;

mod crypto;

/// The `BridgeCrypto` derive. this derive will be help to support crypto field in struct.
/// You need add `bridge-primitives` crate to your crate.
///
/// **is_enable**
/// If not have this attribute, the default it's true.
///
/// Example.
/// ```rust
/// #[derive(BridgeCrypto)]
/// pub struct Foo {
///     #[crypto(is_enable)]
///     enable: bool,
///
///     #[crypto(decrypt)]
///     name: String,
///     #[crypto(decrypt)]
///     country: String,
///     power_level: u64,
/// }
/// ```
/// This will be expand to
/// ```rust
/// pub struct Foo {
///     #[crypto(is_enable)]
///     enable: bool,
///     #[crypto(decrypt)]
///     name: String,
///     #[crypto(decrypt)]
///     country: String,
///     #[crypto(decrypt)]
///     power_level: u64,
/// }
/// impl Foo {
///     pub fn name_decrypt(&self, password: impl AsRef<str>) -> anyhow::Result<String> {
///         if !self.enable {
///             return Ok(self.name.clone());
///         }
///         let crypto = bridge_primitives::crypto::Crypto::new();
///         crypto.decrypt(password.as_ref(), &self.name)
///     }
///     pub fn country_decrypt(&self, password: impl AsRef<str>) -> anyhow::Result<String> {
///         if !self.enable {
///             return Ok(self.country.clone());
///         }
///         let crypto = bridge_primitives::crypto::Crypto::new();
///         crypto.decrypt(password.as_ref(), &self.country)
///     }
/// }
/// ```
///
#[proc_macro_derive(BridgeCrypto, attributes(crypto))]
pub fn derive_bridge_crypto(input: TokenStream) -> TokenStream {
    crypto::crypto(input)
}
