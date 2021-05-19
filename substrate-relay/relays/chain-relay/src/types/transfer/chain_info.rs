use crate::CliChain;
use getset::{Getters, MutGetters, Setters};

#[derive(Debug, Clone, Default, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct ChainInfo {
	name: String,
	host: String,
	port: u32,
	signer: String,
	secure: bool,
	signer_password: Option<String>,
}

impl ChainInfo {
	/// Convert connection params into Substrate client.
	pub async fn to_substrate_relay_chain<C: CliChain>(&self) -> anyhow::Result<relay_substrate_client::Client<C>> {
		Ok(
			relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
				host: self.host.clone(),
				port: self.port.clone() as u16,
				secure: self.secure,
			})
			.await,
		)
	}

	/// Parse signing params into chain-specific KeyPair.
	pub fn to_keypair<C: CliChain>(&self) -> anyhow::Result<C::KeyPair> {
		use sp_core::crypto::Pair;

		C::KeyPair::from_string(&self.signer, self.signer_password.as_deref())
			.map_err(|e| anyhow::format_err!("{:?}", e))
	}
}
