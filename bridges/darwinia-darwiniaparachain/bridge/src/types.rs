use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use support_common::error::BridgerError;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum BridgeName {
    PolkadotToDarwinia,
    DarwiniaToDarwiniaParachain,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HexLaneId(pub [u8; 4]);

impl FromStr for HexLaneId {
    type Err = BridgerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = array_bytes::hex2array(s)
            .map_err(|e| BridgerError::Hex(format!("Failed to parse lane id: {:?}", e)))?;
        Ok(HexLaneId(hex))
    }
}

impl Display for HexLaneId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let lane_id = self.0;
        let hex = array_bytes::bytes2hex("0x", lane_id.as_ref());
        f.write_str(&hex[..])
    }
}

impl<'de> Deserialize<'de> for HexLaneId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_text: String = Deserialize::deserialize(deserializer)?;
        let lane = HexLaneId::from_str(&hex_text[..]).map_err(serde::de::Error::custom)?;
        Ok(lane)
    }
}

impl Serialize for HexLaneId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let hex_text = self.to_string();
        serializer.serialize_str(&hex_text[..])
    }
}
