use crate::persist;
use relay_chain::types::transfer::ChainInfo;

impl persist::Chain {
	pub fn to_chain_info(&self) -> ChainInfo {
		let mut chain_info = ChainInfo::default();
		chain_info.set_name(self.name.clone());
		chain_info.set_host(self.host.clone());
		chain_info.set_port(self.port);
		chain_info.set_signer(self.signer.clone());
		chain_info.set_secure(self.secure);
		chain_info.set_signer_password(self.signer_password.clone());
		chain_info
	}
}
