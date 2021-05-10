use actix_web::client::Client;
use colored::*;
use getset::{Getters, Setters};
use typed_builder::TypedBuilder;

use crate::error;
use crate::persist::Chain;
use crate::types::patch::resp::Resp;

#[derive(Debug, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct CliClient {
	debug: bool,
	server: String,
	token: Option<String>,
}

impl CliClient {
	pub fn new(server: String, token: Option<String>, debug: bool) -> Self {
		Self {
			server,
			token,
			debug,
		}
	}

	pub fn api<T: AsRef<str>>(&self, path: T) -> String {
		format!("{}/api/chain/list", self.server)
	}

	pub fn client(&self) -> Client {
		Client::builder()
			.bearer_auth(self.token.clone().unwrap_or("".to_string()))
			.finish()
	}
}

// chain
impl CliClient {
	pub async fn chain_list(&self) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/chain/list");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.get(&api)
			.send()
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Vec<Chain>> = response.json().await?;
		let chains = resp.ok_or_else(|msg| {
			error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string()))
		})?;
		println!(
			"{}\t\t{}\t\t{}\t\t{}",
			"name".bold(),
			"host".bold(),
			"port".bold(),
			"signer".bold()
		);
		chains.iter().for_each(|chain| {
			println!(
				"{}\t\t{}\t\t{}\t\t{}",
				chain.name(),
				chain.host(),
				chain.port(),
				chain.signer()
			);
		});
		// debug!("{:?}", chains);
		Ok(())
	}
	pub async fn chain_add(&self, chain: &Chain) -> error::Result<()> {
		Ok(())
	}
	pub async fn chain_update(&self, chain: &Chain) -> error::Result<()> {
		Ok(())
	}
	pub async fn chain_remove<T: AsRef<str>>(&self, chain_name: T) -> error::Result<()> {
		Ok(())
	}
}

// token
impl CliClient {
	pub async fn token_list(&self) -> error::Result<()> {
		Ok(())
	}
	pub async fn token_generate(&self, remark: &Option<String>) -> error::Result<()> {
		Ok(())
	}
	pub async fn token_remove<T: AsRef<str>>(&self, token: T) -> error::Result<()> {
		Ok(())
	}
}
