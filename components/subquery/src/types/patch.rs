use serde::de::Error;
use serde::{Deserialize, Deserializer};

pub(super) fn smart_vec_string_to_vec_hex<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Vec::<String>::deserialize(deserializer)?;
    let mut r = Vec::with_capacity(s.len());
    for v in s {
        match hex::decode(v) {
            Ok(v) => r.push(v),
            Err(e) => {
                return Err(D::Error::custom(format!(
                    "can't deserialize to Vec<Vec<u8>>: {}",
                    e
                )))
            }
        }
    }
    Ok(r)
}
