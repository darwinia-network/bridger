use crate::types::general::Param;
use serde::{Deserialize, Deserializer};

pub(crate) fn smart_deserialize_param<'de, D>(deserializer: D) -> Result<Vec<Param>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(json) = String::deserialize(deserializer.clone()) {
        return serde_json::from_str(&json);
    }
    Vec::<Param>::deserialize(deserializer)
}
