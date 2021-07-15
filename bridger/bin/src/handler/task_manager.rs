use std::ffi::OsStr;
use std::path::PathBuf;

use lifeline::CarryFrom;

use bridge_traits::bridge::task::{BridgeSand, BridgeTask};
use bridge_traits::error::StandardError;
use linked_darwinia::config::DarwiniaLinkedConfig;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_ethereum::config::DarwiniaEthereumConfig;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_millau::config::PangolinMillauConfig;
use task_pangolin_millau::task::PangolinMillauTask;

use crate::types::transfer::{TaskConfigTemplateParam, TaskStartParam};

fn task_config<T: serde::de::DeserializeOwned>(path_config: PathBuf) -> anyhow::Result<T> {
    let mut c = config::Config::default();
    c.merge(config::File::from(path_config))?;
    let tc = c.try_into::<T>().map_err(|e| {
        StandardError::Api(format!(
            "Failed to load task config: {:?} of path: {:?}",
            e, path_config
        ))
    })?;
    Ok(tc)
}

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
                format: format.to_string(),
                name: task.clone(),
                config: None,
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
                format: format.to_string(),
                name: task.clone(),
                config: None,
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

    let config_format = &param.format;
    let option_config = &param.config;

    if !support_keep::task::is_available_task(name) {
        return Err(StandardError::Api(format!("Not support this task [{}]", &param.name)).into());
    }
    let path_config = base_path.join(format!("{}.{}", name, config_format));
    if let Some(config_raw) = option_config {
        tokio::fs::write(&path_config, &config_raw).await?
    }
    if !path_config.exists() {
        return Err(
            StandardError::Api(format!("The config file not found: {:?}", path_config)).into(),
        );
    }

    let state_bridge = support_keep::state::get_state()
        .ok_or_else(|| StandardError::Api("Please set bridge state first.".to_string()))?;

    match name {
        DarwiniaLinked::NAME => {
            let task_config = task_config::<DarwiniaLinkedConfig>(path_config)?;
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
            let task_config = task_config::<DarwiniaEthereumConfig>(path_config)?;
            let mut task = DarwiniaEthereumTask::new(task_config, state_bridge.clone()).await?;

            let linked_darwinia: &DarwiniaLinked =
                support_keep::task::running_task_cast(DarwiniaLinked::NAME)?;
            task.keep_carry(linked_darwinia.bus().carry_from(task.bus())?);

            support_keep::task::keep_task(DarwiniaEthereumTask::NAME, Box::new(task))?;
        }
        PangolinMillauTask::NAME => {
            let task_config = task_config::<PangolinMillauConfig>(path_config)?;
            let task = PangolinMillauTask::new(task_config).await?;
            support_keep::task::keep_task(PangolinMillauTask::NAME, Box::new(task))?;
        }
        _ => return Err(StandardError::Api(format!("Unsupported task: [{}]", name)).into()),
    };

    Ok(())
}

pub fn task_config_template(param: TaskConfigTemplateParam) -> anyhow::Result<String> {
    let task_name = param.name;
    let format = param.format;
    if !support_keep::task::is_available_task(&task_name) {
        return Err(StandardError::Api(format!("Not support this task [{}]", &task_name)).into());
    }
    let value = match &task_name[..] {
        DarwiniaLinked::NAME => DarwiniaLinked::config_template(),
        DarwiniaEthereumTask::NAME => DarwiniaEthereumTask::config_template(),
        PangolinMillauTask::NAME => PangolinMillauTask::config_template(),
        _ => {
            return Err(StandardError::Api(format!(
                "Unsupported to show default config template: [{}]",
                task_name
            ))
            .into())
        }
    }?;
    let template = match &format[..] {
        "toml" => toml::to_string(&value)?,
        "json" => serde_json::to_string_pretty(&value)?,
        "yml" => serde_yaml::to_string(&value)?,
        _ => {
            return Err(
                StandardError::Api(format!("Unsupported this config format: [{}]", format)).into(),
            )
        }
    };
    Ok(template)
}
