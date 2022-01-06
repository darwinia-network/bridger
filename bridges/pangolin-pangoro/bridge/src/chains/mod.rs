pub mod pangolin;
pub mod pangoro;

// Millau/Rialto tokens have no any real value, so the conversion rate we use is always 1:1. But we want to
// test our code that is intended to work with real-value chains. So to keep it close to 1:1, we'll be treating
// Rialto as BTC and Millau as wBTC (only in relayer).

/// The identifier of token, which value is associated with Rialto token value by relayer.
pub(crate) const PANGOLIN_ASSOCIATED_TOKEN_ID: &str = "bitcoin";
/// The identifier of token, which value is associated with Millau token value by relayer.
pub(crate) const PANGORO_ASSOCIATED_TOKEN_ID: &str = "wrapped-bitcoin";
