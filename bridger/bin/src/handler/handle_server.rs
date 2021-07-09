use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use lifeline::CarryFrom;
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask};
use bridge_traits::error::StandardError;
use component_state::config::{BridgeStateConfig, MicrokvConfig};
use component_state::state::BridgeStateComponent;
use linked_darwinia::config::DarwiniaLinkedConfig;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_ethereum::task::{DarwiniaEthereumConfig, DarwiniaEthereumTask};
use task_pangolin_millau::task::{PangolinMillauConfig, PangolinMillauTask};

use crate::patch;
use crate::types::command::ServerOptions;
use crate::types::server::{Resp, WebserverState};
use crate::types::transfer::{TaskListResponse, TaskStartParam, TaskStopParam};

/// Handler bridger server
pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
    start_webserver(options).await?;
    Ok(())
}

async fn start_webserver(options: ServerOptions) -> anyhow::Result<()> {
    let router = router(options.clone()).await;

    let service = RouterService::new(router).unwrap();

    let host_port = format!("{}:{}", options.host, options.port);
    let addr: SocketAddr = host_port.parse()?;

    let server = Server::bind(&addr).serve(service);

    log::info!("darwinia-bridger is running on: {}", addr);
    if let Err(err) = server.await {
        log::error!("Server error: {}", err);
    }
    Ok(())
}

/// Define routerify router
async fn router(options: ServerOptions) -> Router<Body, anyhow::Error> {
    let state = app_state(options).await.expect("Failed to build app state");
    Router::builder()
        .data(state)
        .middleware(Middleware::pre(logger))
        .get("/", hello)
        .get("/task/list", task_list)
        .post("/task/start", task_start)
        .post("/task/stop", task_stop)
        .post("/task/:task_name/:task_route", task_route)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

/// Routerify app state, include bridger common config
async fn app_state(options: ServerOptions) -> anyhow::Result<WebserverState> {
    let base_path = patch::bridger::base_path(options.base_path)?;

    let config_state = BridgeStateConfig {
        microkv: MicrokvConfig {
            base_path: base_path.clone(),
            db_name: Some("database".to_string()),
            auto_commit: true,
        },
    };
    let component_state = BridgeStateComponent::new(config_state);
    let bridge_state = component_state.component().await?;
    support_keep::state::set_state(bridge_state)?;

    Ok(WebserverState {
        base_path: Arc::new(base_path),
    })
}

/// Routerify access log
async fn logger(req: Request<Body>) -> anyhow::Result<Request<Body>> {
    log::debug!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

/// Handle routerify error
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    log::error!("{:?}", err);
    let msg = format!("{}", err);
    Resp::<String>::err_with_msg(msg)
        .response_json()
        .expect("Failed to build response")
}

/// Index
async fn hello(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    Ok(Resp::<String>::ok().response_json()?)
}

/// Get task list
async fn task_list(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
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

fn task_config<T: serde::de::DeserializeOwned>(path_config: PathBuf) -> anyhow::Result<T> {
    let mut c = config::Config::default();
    c.merge(config::File::from(path_config))?;
    let tc = c
        .try_into::<T>()
        .map_err(|e| StandardError::Api(format!("Failed to load task config: {:?}", e)))?;
    Ok(tc)
}

/// Start a task
async fn task_start(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStartParam = patch::hyper::deserialize_body(&mut req).await?;

    let name = &param.name[..];
    if support_keep::task::task_is_running(name) {
        return Resp::<String>::ok_with_msg(format!("The task [{}] is running", &param.name))
            .response_json();
    }

    let state_webserver = req.data::<WebserverState>().unwrap();

    let config_format = &param.format;
    let option_config = &param.config;

    if !support_keep::task::is_available_task(name) {
        return Resp::<String>::err_with_msg(format!("Not support this task [{}]", &param.name))
            .response_json();
    }
    let path_config = state_webserver
        .base_path
        .join(format!("{}.{}", name, config_format));
    if let Some(config_raw) = option_config {
        tokio::fs::write(&path_config, &config_raw).await?
    }
    if !path_config.exists() {
        return Resp::<String>::err_with_msg(format!(
            "The config file not found: {:?}",
            path_config
        ))
        .response_json();
    }

    let state_bridge = support_keep::state::get_state()
        .ok_or_else(|| StandardError::Api("Please set bridge state first.".to_string()))?;

    // let mut task_router = TaskRouter::new();
    match name {
        DarwiniaLinked::NAME => {
            let task_config = task_config::<DarwiniaLinkedConfig>(path_config)?;
            let task = DarwiniaLinked::new(task_config).await?;
            // task.register_route(&mut task_router);
            support_keep::task::keep_task(DarwiniaLinked::NAME, Box::new(task))?;
        }
        DarwiniaEthereumTask::NAME => {
            if !support_keep::task::task_is_running(DarwiniaLinked::NAME) {
                return Resp::<String>::err_with_msg(format!(
                    "Please start [{}] first",
                    DarwiniaLinked::NAME
                ))
                .response_json();
            }
            let task_config = task_config::<DarwiniaEthereumConfig>(path_config)?;
            let mut task = DarwiniaEthereumTask::new(task_config, state_bridge.clone()).await?;

            let linked_darwinia: &DarwiniaLinked =
                support_keep::task::running_task_cast(DarwiniaLinked::NAME)?;
            task.keep_carry(linked_darwinia.bus().carry_from(task.bus())?);

            // task.register_route(&mut task_router);
            support_keep::task::keep_task(DarwiniaEthereumTask::NAME, Box::new(task))?;
        }
        PangolinMillauTask::NAME => {
            let task_config = task_config::<PangolinMillauConfig>(path_config)?;
            let task = PangolinMillauTask::new(task_config).await?;
            // task.register_route(&mut task_router);
            support_keep::task::keep_task(PangolinMillauTask::NAME, Box::new(task))?;
        }
        _ => unreachable!(),
    };

    // let custom_router = task_router.router();
    // support_keep::route::merge_route(custom_router)?;

    Resp::<String>::ok().response_json()
}

/// Start a task
async fn task_stop(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStopParam = patch::hyper::deserialize_body(&mut req).await?;
    log::debug!("{:?}", param);
    let task_name = param.name;
    support_keep::task::stop_task(&task_name)?;
    log::warn!("The task {} is stopped", task_name);
    Resp::<String>::ok().response_json()
}

async fn task_route(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    // let task_name = req
    //     .param("task_name")
    //     .ok_or_else(|| StandardError::Api("The task name is required".to_string()))?;
    // let task_route = req
    //     .param("task_route")
    //     .ok_or_else(|| StandardError::Api("The task route is required".to_string()))?;
    //
    // let uri = format!("{}-{}", task_name, task_route);
    // let param: serde_json::Value = patch::hyper::deserialize_body(&mut req)
    //     .await
    //     .unwrap_or(serde_json::Value::Null);
    //
    // let value = support_keep::route::run_route(uri, param).await?;
    // Resp::ok_with_data(value).response_json()

    let task_name = req
        .param("task_name")
        .ok_or_else(|| StandardError::Api("The task name is required".to_string()))?;
    let task_route = req
        .param("task_route")
        .ok_or_else(|| StandardError::Api("The task route is required".to_string()))?;
    let uri = format!("{}-{}", task_name, task_route);
    let param: serde_json::Value = patch::hyper::deserialize_body(&mut req)
        .await
        .unwrap_or(serde_json::Value::Null);

    // let task = support_keep::task::running_task(task_name)
    //     .ok_or_else(|| StandardError::Api("The task isn't started".to_string()))?;
    // let value = task.route(uri, param).await?;
    //
    // Resp::ok_with_data(value).response_json()

    Resp::<String>::ok().response_json()
}
