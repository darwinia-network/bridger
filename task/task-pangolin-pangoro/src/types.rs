use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bp_messages::LaneId;
use messages_relay::message_lane_loop::RelayerMode;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::traits::CliChain;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
pub enum BridgeName {
    #[strum(serialize = "pangolin-to-pangoro")]
    PangolinToPangoro,
    #[strum(serialize = "pangoro-to-pangolin")]
    PangoroToPangolin,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfo {
    pub host: String,
    pub port: u16,
    pub signer: Option<String>,
    pub secure: bool,
    pub signer_password: Option<String>,
}

impl ChainInfo {
    /// Convert connection params into Substrate client.
    pub async fn to_substrate_relay_chain<C: CliChain>(
        &self,
    ) -> anyhow::Result<relay_substrate_client::Client<C>> {
        Ok(
            relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
                host: self.host.clone(),
                port: self.port,
                secure: self.secure,
            })
            .await,
        )
    }

    /// Parse signing params into chain-specific KeyPair.
    pub fn to_keypair<C: CliChain>(&self) -> anyhow::Result<C::KeyPair> {
        use sp_core::crypto::Pair;

        let signer = match self.signer.clone() {
            Some(v) => v,
            None => anyhow::bail!("The chain [{}:{}] not set signer", self.host, self.port,),
        };
        C::KeyPair::from_string(&signer, self.signer_password.as_deref())
            .map_err(|e| anyhow::format_err!("{:?}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitBridge {
    pub bridge: BridgeName,
    pub source: ChainInfo,
    pub target: ChainInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HexLaneId(pub LaneId);

impl From<HexLaneId> for LaneId {
    fn from(lane_id: HexLaneId) -> LaneId {
        lane_id.0
    }
}

impl std::str::FromStr for HexLaneId {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lane_id = LaneId::default();
        hex::decode_to_slice(s, &mut lane_id)?;
        Ok(HexLaneId(lane_id))
    }
}

impl Display for HexLaneId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let lane_id = self.0;
        let hex_text = hex::encode(lane_id);
        f.write_str(&hex_text[..])
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrometheusParamsInfo {
    /// Do not expose a Prometheus metric endpoint.
    pub no_prometheus: bool,
    /// Expose Prometheus endpoint at given interface.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub prometheus_host: String,
    /// Expose Prometheus endpoint at given port.
    pub prometheus_port: u16,
}

impl From<PrometheusParamsInfo> for relay_utils::metrics::MetricsParams {
    fn from(cli_params: PrometheusParamsInfo) -> relay_utils::metrics::MetricsParams {
        if !cli_params.no_prometheus {
            Some(relay_utils::metrics::MetricsAddress {
                host: cli_params.prometheus_host,
                port: cli_params.prometheus_port,
            })
            .into()
        } else {
            None.into()
        }
    }
}

/// Relayer operating mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
pub enum WrapperRelayerMode {
    /// The relayer doesn't care about rewards.
    #[strum(serialize = "altruistic")]
    Altruistic,
    /// The relayer will deliver all messages and confirmations as long as he's not losing any funds. default value
    #[strum(serialize = "rational")]
    Rational,
}

impl From<WrapperRelayerMode> for RelayerMode {
    fn from(wrm: WrapperRelayerMode) -> Self {
        match wrm {
            WrapperRelayerMode::Altruistic => RelayerMode::Altruistic,
            WrapperRelayerMode::Rational => RelayerMode::Rational,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RelayHeadersAndMessagesInfo {
    pub bridge: BridgeName,

    pub source: ChainInfo,
    pub target: ChainInfo,

    pub lanes: Vec<HexLaneId>,
    pub prometheus_params: PrometheusParamsInfo,
    pub relayer_mode: RelayerMode,
}
