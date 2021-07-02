use bridge_standard::error::StandardError;

use crate::patch;
use crate::types::command::TaskCommand;
use crate::types::server::Resp;
use crate::types::transfer::{TaskListResponse, TaskStartParam, TaskStopParam};

pub async fn handle_task(server: String, command: TaskCommand) -> anyhow::Result<()> {
    match command {
        TaskCommand::List => {
            let resp = reqwest::get(format!("{}/task/list", server))
                .await?
                .json::<Resp<Vec<TaskListResponse>>>()
                .await?;
            if resp.is_err() {
                return Err(StandardError::Cli(resp.msg().to_string()).into());
            }
            if let Some(tasks) = resp.data() {
                tasks.iter().for_each(|task| {
                    if task.running {
                        println!("RUNNING {}", task.name);
                    } else {
                        println!("STOPPED {}", task.name);
                    }
                });
            }
        }
        TaskCommand::Start {
            name,
            format,
            config,
        } => {
            if !patch::bridger::is_allow_config_format(&format) {
                eprintln!("Not support this format. {}", format);
                return Ok(());
            }
            let content = match config {
                Some(path) => Some(tokio::fs::read_to_string(&path).await?),
                None => None,
            };
            let param = TaskStartParam {
                format,
                name,
                config: content,
            };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/task/start", server))
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
        TaskCommand::Stop { name } => {
            let param = TaskStopParam { name };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/task/stop", server))
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
    };
    Ok(())
}
