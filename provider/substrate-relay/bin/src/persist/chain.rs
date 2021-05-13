use relay_chain::CliChain;

use crate::error;
use crate::persist;

impl persist::Chain {
	/// Convert connection params into Substrate client.
	pub async fn to_substrate_relay_chain<C: CliChain>(&self) -> error::Result<relay_substrate_client::Client<C>> {
		Ok(
			relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
				host: self.host.clone(),
				port: self.port.clone() as u16,
				secure: self.secure,
			})
			.await?,
		)
	}

	/// Parse signing params into chain-specific KeyPair.
	pub fn to_keypair<C: CliChain>(&self) -> error::Result<C::KeyPair> {
		use sp_core::crypto::Pair;

		C::KeyPair::from_string(&self.signer, self.signer_password.as_deref())
			.map_err(|e| anyhow::format_err!("{:?}", e))
	}
}
