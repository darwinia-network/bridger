use std::path::Path;

use bridge_standard::bridge::task::BridgeSand;
use bridge_standard::error::StandardError;
use task_darwinia_ethereum::task::{DarwiniaEthereumConfig, DarwiniaEthereumTask};
use task_pangolin_millau::task::PangolinMillauTask;

use crate::dc;
use crate::types::command::TaskCommand;

pub async fn handle_task(command: TaskCommand) -> anyhow::Result<()> {
    match command {
        TaskCommand::List => {
            let tasks = dc::available_tasks()?;
            tasks.iter().for_each(|item| println!("{}", item));
        }
        TaskCommand::Start { name, config } => {
            let path = Path::new(&config);
            let shared = dc::get_shared().ok_or(StandardError::Cli(
                "The shared service isn't start, please start it first.".to_string(),
            ))?;
            let channel = shared.channel();
            match &name[..] {
                DarwiniaEthereumTask::NAME => {
                    let mut c = config::Config::default();
                    c.merge(config::File::from(path))?;
                    let shared_config = c.try_into::<DarwiniaEthereumConfig>().map_err(|e| {
                        StandardError::Cli(format!(
                            "Failed to load darwina-ethereum config: {:?}",
                            e
                        ))
                    })?;
                    let task = DarwiniaEthereumTask::new(shared_config, channel).await?;
                    dc::keep_task(DarwiniaEthereumTask::NAME, Box::new(task))?;
                    println!("Start {} success", DarwiniaEthereumTask::NAME);
                }
                PangolinMillauTask::NAME => {
                    println!("start PangolinMillauTask");
                }
                _ => anyhow::bail!("Not support this task: [{}]", name),
            };
        }
        TaskCommand::Stop { name } => {
            println!("stop task: {}", name)
        }
    };
    Ok(())
}

#[warn(dead_code)]
async fn start_shared() -> anyhow::Result<()> {
    // let shared = BridgeShared::new(self::config_shared())?;
    // let channel = shared.channel();
    // Ok(channel)
    Ok(())
}
