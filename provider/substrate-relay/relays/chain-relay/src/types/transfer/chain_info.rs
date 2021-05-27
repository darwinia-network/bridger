use crate::CliChain;
use getset::{Getters, MutGetters, Setters};

#[derive(Debug, Clone, Default, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct ChainInfo {
	host: String,
	port: u32,
	signer: Option<String>,
	secure: bool,
	signer_password: Option<String>,
}

impl ChainInfo {
	pub fn new(
		entrypoint: String,
		signer: Option<String>,
		signer_password: Option<String>,
	) -> anyhow::Result<Self> {
		if entrypoint.find("ws://").unwrap_or(usize::MAX) != 0
			&& entrypoint.find("wss://").unwrap_or(usize::MAX) != 0
		{
			anyhow::bail!("The entrypoint isn't websocket protocol")
		}
		let secure = entrypoint.starts_with("wss://");
		let entrypoint = entrypoint
			.replace(if secure { "wss://" } else { "ws://" }, "")
			.replace("/", "")
			.replace(" ", "");
		let host_port = entrypoint.split(':').collect::<Vec<&str>>();
		let host = host_port.get(0).unwrap_or(&"127.0.0.1");
		let port = host_port
			.get(1)
			.unwrap_or_else(|| if secure { &"443" } else { &"80" });
		Ok(Self {
			host: host.to_string(),
			port: port.parse::<u32>()?,
			signer,
			secure,
			signer_password,
		})
	}
}

impl ChainInfo {
	/// Convert connection params into Substrate client.
	pub async fn to_substrate_relay_chain<C: CliChain>(
		&self,
	) -> anyhow::Result<relay_substrate_client::Client<C>> {
		Ok(
			relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
				host: self.host.clone(),
				port: self.port as u16,
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
			None => anyhow::bail!("The chain [{}:{}] not set signer", self.host, self.port),
		};
		C::KeyPair::from_string(&signer, self.signer_password.as_deref())
			.map_err(|e| anyhow::format_err!("{:?}", e))
	}
}
