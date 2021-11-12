use crate::types::general::Param;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

pub(crate) fn smart_deserialize_param<'de, D>(deserializer: D) -> Result<Vec<Param>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeVecParamFromStringOrVecVisitor)
}

struct DeserializeVecParamFromStringOrVecVisitor;

impl<'de> serde::de::Visitor<'de> for DeserializeVecParamFromStringOrVecVisitor {
    type Value = Vec<Param>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an string or array")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::from_str(v).map_err(|e| E::invalid_value(Unexpected::Str(v), &self))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Param>::deserialize(deserializer)
    }
}
