use async_recursion::async_recursion;
use colored::Colorize;

use bridge_standard::error::StandardError;

use crate::patch;
use crate::types::command::TaskCommand;
use crate::types::server::Resp;
use crate::types::transfer::{TaskListResponse, TaskStartParam, TaskStopParam};

#[async_recursion]
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
                        println!("{} {}", "RUNNING".green(), task.name);
                    } else {
                        println!("{} {}", "STOPPED".red(), task.name);
                    }
                });
            }
        }
        TaskCommand::Start { options } => {
            let format = options.format;
            let name = options.name;
            let config = options.config;
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
        TaskCommand::Restart { options } => {
            handle_task(
                server.clone(),
                TaskCommand::Stop {
                    name: options.name.clone(),
                },
            )
            .await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            handle_task(server, TaskCommand::Start { options }).await?
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
