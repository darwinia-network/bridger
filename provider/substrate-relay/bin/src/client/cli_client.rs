use actix_web::client::Client;
use colored::*;
use getset::{Getters, Setters};
use typed_builder::TypedBuilder;

use crate::error;
use crate::persist::{Chain, Token};
use crate::types::cond::chain::ChainRemoveCond;
use crate::types::cond::relay::InitBridgeCond;
use crate::types::cond::token::{TokenGenerateCond, TokenRemoveCond};
use crate::types::patch::resp::Resp;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct CliClient {
	debug: bool,
	server: String,
	token: Option<String>,
}

impl CliClient {
	pub fn new(server: String, token: Option<String>, debug: bool) -> Self {
		Self { server, token, debug }
	}

	pub fn api<T: AsRef<str>>(&self, path: T) -> String {
		format!("{}{}", self.server, path.as_ref())
	}

	pub fn client(&self) -> Client {
		Client::builder()
			.connector(
				actix_web::client::Connector::new()
					.timeout(Duration::from_secs(30))
					.finish(),
			)
			.timeout(Duration::from_secs(30))
			.bearer_auth(self.token.clone().unwrap_or("".to_string()))
			.finish()
	}
}

// print
impl CliClient {
	fn show_chains(&self, chains: &Vec<Chain>) {
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
	}
	fn show_tokens(&self, tokens: &Vec<Token>) {
		println!("{}\t\t{}", "token".bold(), "remark".bold(),);
		tokens.iter().for_each(|token| {
			println!(
				"{}\t\t{}",
				token.value(),
				token.remark().clone().unwrap_or("".to_string())
			);
		});
	}
	fn show_token(&self, token: &Token) {
		self.show_tokens(&vec![token.clone()])
	}
	fn show_success(&self) {
		println!("{}", "Success".green())
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
		let chains = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_chains(&chains);
		Ok(())
	}
	pub async fn chain_add(&self, chain: &Chain) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/chain/add");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(chain)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Vec<Chain>> = response.json().await?;
		let chains = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_chains(&chains);
		Ok(())
	}
	pub async fn chain_update(&self, chain: &Chain) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/chain/update");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(chain)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Vec<Chain>> = response.json().await?;
		let chains = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_chains(&chains);
		Ok(())
	}
	pub async fn chain_remove(&self, chain_remove: &ChainRemoveCond) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/chain/remove");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(chain_remove)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Vec<Chain>> = response.json().await?;
		let chains = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_chains(&chains);
		Ok(())
	}
}

// token
impl CliClient {
	pub async fn token_list(&self) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/token/list");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.get(&api)
			.send()
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Vec<Token>> = response.json().await?;
		let tokens = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_tokens(&tokens);
		Ok(())
	}
	pub async fn token_generate(&self, token_generate: &TokenGenerateCond) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/token/generate");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(token_generate)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<Token> = response.json().await?;
		let token = resp.ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_token(&token);
		Ok(())
	}
	pub async fn token_remove(&self, token_remove: &TokenRemoveCond) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/token/remove");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(token_remove)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<String> = response.json().await?;
		let _ = resp.safe_ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_success();
		Ok(())
	}
}

// relay
impl CliClient {
	pub async fn init_bridge(&self, init_bridge: &InitBridgeCond) -> error::Result<()> {
		let mut client = self.client();
		let api = self.api("/api/relay/init-bridge");
		if self.debug {
			println!("{} {}", "API:".green().bold(), api);
		}
		let mut response = client
			.post(&api)
			.send_form(init_bridge)
			.await
			.map_err(|e| error::CliError::RequestError(e.to_string()))?;
		let resp: Resp<String> = response.json().await?;
		let _ = resp.safe_ok_or_else(|msg| error::CliError::ApiError(api, msg.unwrap_or("unknown".to_string())))?;
		self.show_success();
		Ok(())
	}
}
