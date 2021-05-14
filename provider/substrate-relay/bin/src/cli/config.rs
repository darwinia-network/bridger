use crate::cli::types::{OptChainCommand, OptConfig, OptConfigSubcommand, OptTokenCommand};
use crate::client::cli_client::CliClient;
use crate::error;
use crate::persist::Chain;
use crate::types::cond::chain::ChainRemoveCond;
use crate::types::cond::token::{TokenGenerateCond, TokenRemoveCond};

pub async fn exec(config: OptConfig) -> error::Result<()> {
	let debug: bool = *(config.debug());
	let server: &String = config.server();
	let token: &Option<String> = config.token();
	let sub: &OptConfigSubcommand = config.sub_command();
	let client = CliClient::new(server.clone(), token.clone(), debug);
	match sub {
		OptConfigSubcommand::Chain(command_chain) => handle_command_chain(&client, command_chain).await?,
		OptConfigSubcommand::Token(command_token) => handle_command_token(&client, command_token).await?,
	}
	Ok(())
}

async fn handle_command_chain(client: &CliClient, command: &OptChainCommand) -> error::Result<()> {
	match command {
		OptChainCommand::List => client.chain_list().await?,
		OptChainCommand::Add { chain_info } => {
			let chain = chain_info.clone().into();
			client.chain_add(&chain).await?
		}
		OptChainCommand::Update { chain_info } => {
			let chain = chain_info.clone().into();
			client.chain_update(&chain).await?
		}
		OptChainCommand::Remove { name } => {
			let chain_remove = ChainRemoveCond::builder().name(name.clone()).build();
			client.chain_remove(&chain_remove).await?
		}
	}
	Ok(())
}

async fn handle_command_token(client: &CliClient, command: &OptTokenCommand) -> error::Result<()> {
	match command {
		OptTokenCommand::List => client.token_list().await?,
		OptTokenCommand::Generate { remark } => {
			let token_generate = TokenGenerateCond::builder().remark(remark.clone()).build();
			client.token_generate(&token_generate).await?
		}
		OptTokenCommand::Remove { token } => {
			let token_remove = TokenRemoveCond::builder().token(token.clone()).build();
			client.token_remove(&token_remove).await?
		}
	}
	Ok(())
}
