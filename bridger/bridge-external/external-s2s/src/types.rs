use bp_messages::LaneId;
use serde::{Deserialize, Serialize};

use crate::traits::CliChain;

// EnumFromStr
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum BridgeName {
    PangolinToMillau,
    MillauToPangolin,
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrometheusParamsInfo {
    /// Do not expose a Prometheus metric endpoint.
    pub no_prometheus: bool,
    /// Expose Prometheus endpoint at given interface.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayHeadersAndMessagesInfo {
    pub bridge: BridgeName,

    pub source: ChainInfo,
    pub target: ChainInfo,

    pub lanes: Vec<HexLaneId>,
    pub prometheus_params: PrometheusParamsInfo,
}
