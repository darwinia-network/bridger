use crate::cli::opt::{OptChainCommand, OptConfig, OptConfigSubcommand, OptTokenCommand};
use crate::client::cli_client::CliClient;
use crate::error;
use crate::persist::Chain;

pub async fn exec(config: OptConfig) -> error::Result<()> {
	let debug: bool = *(config.debug());
	let server: &String = config.server();
	let token: &Option<String> = config.token();
	let sub: &OptConfigSubcommand = config.sub_command();
	let client = CliClient::new(server.clone(), token.clone(), debug);
	match sub {
		OptConfigSubcommand::Chain(command_chain) => {
			handle_command_chain(&client, command_chain).await?
		}
		OptConfigSubcommand::Token(command_token) => {
			handle_command_token(&client, command_token).await?
		}
	}
	Ok(())
}

async fn handle_command_chain(client: &CliClient, command: &OptChainCommand) -> error::Result<()> {
	match command {
		OptChainCommand::List => client.chain_list().await?,
		OptChainCommand::Add {
			name,
			host,
			port,
			signer,
		} => {
			let chain = Chain::builder()
				.name(name.clone())
				.host(host.clone())
				.port(*port)
				.signer(signer.clone())
				.build();
			client.chain_add(&chain).await?
		}
		OptChainCommand::Update {
			name,
			host,
			port,
			signer,
		} => {
			let chain = Chain::builder()
				.name(name.clone())
				.host(host.clone())
				.port(*port)
				.signer(signer.clone())
				.build();
			client.chain_update(&chain).await?
		}
		OptChainCommand::Remove { name } => client.chain_remove(name).await?,
	}
	Ok(())
}

async fn handle_command_token(client: &CliClient, command: &OptTokenCommand) -> error::Result<()> {
	match command {
		OptTokenCommand::List => client.token_list().await?,
		OptTokenCommand::Generate { remark } => client.token_generate(remark).await?,
		OptTokenCommand::Remove { token } => client.token_remove(token).await?,
	}
	Ok(())
}
