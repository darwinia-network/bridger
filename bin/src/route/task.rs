use hyper::{Body, Request, Response};
use routerify::prelude::*;

use bridge_traits::error::StandardError;
use support_keep::types::WebserverState;

use crate::patch;
use crate::route::task_manager;
use crate::types::server::Resp;
use crate::types::transfer::{
    TaskConfigTemplateParam, TaskListResponse, TaskStartParam, TaskStopParam,
};

/// Get task list
pub async fn task_list(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let tasks = support_keep::task::available_tasks()?;
    let data = tasks
        .iter()
        .map(|item| {
            let running = support_keep::task::task_is_running(item);
            TaskListResponse {
                name: item.clone(),
                running,
            }
        })
        .collect::<Vec<TaskListResponse>>();
    Resp::ok_with_data(data).response_json()
}

/// Start a task
pub async fn task_start(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStartParam = patch::hyper::deserialize_body(&mut req).await?;

    let state = req.data::<WebserverState>().unwrap();
    let base_path = &state.base_path.as_ref();
    if let Err(e) = task_manager::start_task_single(base_path.into(), param).await {
        return Resp::<String>::err_with_msg(format!("{}", e)).response_json();
    }
    Resp::<String>::ok().response_json()
}

/// Start a task
pub async fn task_stop(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStopParam = patch::hyper::deserialize_body(&mut req).await?;
    log::debug!("{:?}", param);
    let task_name = param.name;
    support_keep::task::stop_task(&task_name)?;
    log::warn!("The task {} is stopped", task_name);
    Resp::<String>::ok().response_json()
}

pub async fn task_route(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: serde_json::Value = patch::hyper::deserialize_body(&mut req)
        .await
        .unwrap_or(serde_json::Value::Null);

    let task_name = req
        .param("task_name")
        .ok_or_else(|| StandardError::Api("The task name is required".to_string()))?;
    let task_route = req
        .param("task_route")
        .ok_or_else(|| StandardError::Api("The task route is required".to_string()))?;

    let task = support_keep::task::running_task(task_name).ok_or_else(|| {
        StandardError::Api(format!(
            "The task [{}] not found or isn't started",
            task_name
        ))
    })?;
    let value = task.route(task_route.clone(), param).await?;

    Resp::ok_with_data(value).response_json()
}

pub async fn task_config_template(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskConfigTemplateParam = patch::hyper::deserialize_body(&mut req).await?;
    let config_template = crate::route::task_manager::task_config_template(param)?;
    Resp::ok_with_data(config_template).response_json()
}
