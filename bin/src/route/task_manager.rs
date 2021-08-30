use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::config::ConfigFormat;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask};
use bridge_traits::error::StandardError;
use linked_darwinia::task::DarwiniaLinked;
use support_keep::types::TaskState;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_pangoro::task::PangolinPangoroTask;
use task_pangolin_ropsten::task::PangolinRopstenTask;

use crate::types::transfer::{TaskConfigTemplateParam, TaskStartParam};

/// Auto start all configured task
pub async fn auto_start_task(base_path: PathBuf) -> anyhow::Result<()> {
    let available_tasks = support_keep::task::available_tasks()?;
    let read_dir: Vec<PathBuf> = std::fs::read_dir(&base_path)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_file())
        .collect();
    let all_linked = available_tasks
        .iter()
        .filter(|item| item.starts_with("linked"))
        .collect::<Vec<&String>>();
    let not_linked = available_tasks
        .iter()
        .filter(|item| !item.starts_with("linked"))
        .collect::<Vec<&String>>();

    for task in all_linked {
        if let Some(task_config) = read_dir.iter().find(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .starts_with(task)
        }) {
            let format = task_config
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| {
                    StandardError::Api(format!("Failed to extra config format for [{}]", task))
                })?;
            let param = TaskStartParam {
                format: ConfigFormat::from_str(format).map_err(|_e| {
                    StandardError::Api(format!("Failed to extra config format for [{}]", task))
                })?,
                name: task.clone(),
                config: None,
                password: None,
                store_password: false,
            };
            start_task_single(base_path.clone(), param).await?;
        }
    }

    for task in not_linked {
        if let Some(task_config) = read_dir.iter().find(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .starts_with(task)
        }) {
            let format = task_config
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| {
                    StandardError::Api(format!("Failed to extra config format for [{}]", task))
                })?;
            let param = TaskStartParam {
                format: ConfigFormat::from_str(format).map_err(|_e| {
                    StandardError::Api(format!("Failed to extra config format for [{}]", task))
                })?,
                name: task.clone(),
                config: None,
                password: None,
                store_password: false,
            };
            start_task_single(base_path.clone(), param).await?;
        }
    }
    Ok(())
}

/// Start a single task
pub async fn start_task_single(base_path: PathBuf, param: TaskStartParam) -> anyhow::Result<()> {
    let name = &param.name[..];
    if support_keep::task::task_is_running(name) {
        return Err(StandardError::Api(format!("The task [{}] is running", &param.name)).into());
    }

    let config_format = param.format;
    let option_config = &param.config;

    if !support_keep::task::is_available_task(name) {
        return Err(StandardError::Api(format!("Not support this task [{}]", &param.name)).into());
    }
    let path_config = base_path.join(format!("{}.{}", name, config_format.file_extension()));
    if let Some(config_raw) = option_config {
        Config::persist_raw(&path_config, &config_raw)?;
    }
    if !path_config.exists() {
        return Err(
            StandardError::Api(format!("The config file not found: {:?}", path_config)).into(),
        );
    }

    let state_bridge = support_keep::state::get_state_bridge_ok()?;

    // put task password
    if let Some(password) = param.password {
        state_bridge.put_task_config_password(name, password, param.store_password)?;
    }
    match name {
        DarwiniaLinked::NAME => {
            let task_config = Config::load(&path_config)?;
            let task = DarwiniaLinked::new(task_config).await?;
            support_keep::task::keep_task(DarwiniaLinked::NAME, Box::new(task))?;
        }
        DarwiniaEthereumTask::NAME => {
            if !support_keep::task::task_is_running(DarwiniaLinked::NAME) {
                return Err(StandardError::Api(format!(
                    "Please start [{}] first",
                    DarwiniaLinked::NAME
                ))
                .into());
            }
            let task_config = Config::load(&path_config)?;
            let mut task = DarwiniaEthereumTask::new(task_config, state_bridge.clone()).await?;

            let linked_darwinia: &mut DarwiniaLinked =
                support_keep::task::running_task_downcast_mut(DarwiniaLinked::NAME)?;

            // let carry = linked_darwinia.bus().carry_from(task.bus())?;
            // let stack = task.stack();
            // stack.carry(carry)?;

            linked_darwinia.stack().carry_from(task.stack())?;

            support_keep::task::keep_task(DarwiniaEthereumTask::NAME, Box::new(task))?;
        }
        PangolinRopstenTask::NAME => {
            // if !support_keep::task::task_is_running(DarwiniaLinked::NAME) {
            //     return Err(StandardError::Api(format!(
            //         "Please start [{}] first",
            //         DarwiniaLinked::NAME
            //     ))
            //         .into());
            // }
            let task_config = Config::load(&path_config)?;
            let task = PangolinRopstenTask::new(task_config, state_bridge.clone()).await?;

            // let linked_darwinia: &DarwiniaLinked =
            //     support_keep::task::running_task_downcast_ref(DarwiniaLinked::NAME)?;
            // let carry = linked_darwinia.bus().carry_from(task.bus())?;
            // let stack = task.stack();
            // stack.carry(carry)?;

            support_keep::task::keep_task(PangolinRopstenTask::NAME, Box::new(task))?;
        }
        PangolinPangoroTask::NAME => {
            let task_config = Config::load(&path_config)?;
            let task = PangolinPangoroTask::new(task_config).await?;
            support_keep::task::keep_task(PangolinPangoroTask::NAME, Box::new(task))?;
        }
        _ => return Err(StandardError::Api(format!("Unsupported task: [{}]", name)).into()),
    };

    // keep task state
    let state_task = TaskState {
        config_path: path_config.clone(),
        config_format: config_format.clone(),
    };
    support_keep::state::set_state_task(name, state_task)?;

    Ok(())
}

/// Generate task config template
pub fn task_config_template(param: TaskConfigTemplateParam) -> anyhow::Result<String> {
    let task_name = param.name;
    let format = param.format;
    if !support_keep::task::is_available_task(&task_name) {
        return Err(StandardError::Api(format!("Not support this task [{}]", &task_name)).into());
    }
    let value = match &task_name[..] {
        DarwiniaLinked::NAME => DarwiniaLinked::config_template(),
        DarwiniaEthereumTask::NAME => DarwiniaEthereumTask::config_template(),
        PangolinPangoroTask::NAME => PangolinPangoroTask::config_template(),
        _ => {
            return Err(StandardError::Api(format!(
                "Unsupported to show default config template: [{}]",
                task_name
            ))
            .into());
        }
    }?;
    let template = Config::raw_config(value, format)?;
    Ok(template)
}
