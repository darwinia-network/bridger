use crate::patch;
use crate::types::command::SharedCommand;
use crate::types::server::Resp;
use crate::types::transfer::SharedStartParam;

pub async fn handle_shared(server: String, command: SharedCommand) -> anyhow::Result<()> {
    match command {
        SharedCommand::Start { format, config } => {
            if !patch::bridger::is_allow_config_format(&format) {
                eprintln!("Not support this format. {}", format);
                return Ok(());
            }
            let content = match config {
                Some(path) => Some(tokio::fs::read_to_string(&path).await?),
                None => None,
            };
            let param = SharedStartParam {
                format,
                config: content,
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/shared/start", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<String>>()
                .await?;

            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            println!("{}", resp.msg());
        }
    }
    Ok(())
}
