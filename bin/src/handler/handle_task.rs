use std::collections::HashMap;

use colored::Colorize;

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;

use crate::types::command::TaskCommand;
use crate::types::server::Resp;
use crate::types::transfer::{
    TaskConfigTemplateParam, TaskListResponse, TaskStartParam, TaskStopParam,
};

#[allow(clippy::manual_map)]
#[async_recursion::async_recursion]
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
        TaskCommand::Exec { options } => {
            let params = options.param;
            let mut req_body = HashMap::new();
            for param in params {
                if param.is_empty() {
                    continue;
                }
                let pvs = param.split('=').collect::<Vec<&str>>();
                if pvs.len() != 2 {
                    return Err(StandardError::Api("The params length is wrong".to_string()).into());
                }
                let param_name = pvs
                    .get(0)
                    .ok_or_else(|| StandardError::Api("The param name is required".to_string()))?;
                let param_value = pvs
                    .get(1)
                    .ok_or_else(|| StandardError::Api("The param value is required".to_string()))?;
                req_body.insert(param_name.to_string(), param_value.to_string());
            }
            let url = format!("{}/task/{}/{}", server, options.name, options.api);
            let resp = reqwest::Client::builder()
                .build()?
                .post(url)
                .json(&req_body)
                .send()
                .await?
                .json::<Resp<TaskTerminal>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(tt) => println!("{}", tt.view()),
                None => println!(),
            }
        }
        TaskCommand::ConfigTemplate { name, format } => {
            let param = TaskConfigTemplateParam { name, format };
            let resp = reqwest::Client::builder()
                .build()?
                .post(format!("{}/task/config-template", server))
                .json(&param)
                .send()
                .await?
                .json::<Resp<String>>()
                .await?;
            if resp.is_err() {
                eprintln!("{}", resp.msg());
                return Ok(());
            }
            match resp.data() {
                Some(v) => println!("{}", v),
                None => println!("Not have default template"),
            }
        }
    };
    Ok(())
}
