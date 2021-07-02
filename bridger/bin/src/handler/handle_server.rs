use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_shared::shared::{BridgeShared, SharedConfig, SharedTask};
use bridge_standard::bridge::task::BridgeSand;
use bridge_standard::error::StandardError;
use task_darwinia_ethereum::task::{DarwiniaEthereumConfig, DarwiniaEthereumTask};
use task_pangolin_millau::task::{PangolinMillauConfig, PangolinMillauTask};

use crate::types::command::ServerOptions;
use crate::types::server::{BridgeState, Resp};
use crate::types::transfer::{SharedStartParam, TaskListResponse, TaskStartParam, TaskStopParam};
use crate::{dc, patch};

/// Handler bridger server
pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
    start_webserver(options).await?;
    Ok(())
}

async fn start_webserver(options: ServerOptions) -> anyhow::Result<()> {
    let router = router(options.clone());

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
fn router(options: ServerOptions) -> Router<Body, anyhow::Error> {
    let state = app_state(options).expect("Failed to build app state");
    Router::builder()
        .data(state)
        .middleware(Middleware::pre(logger))
        .get("/", hello)
        .post("/shared/start", start_shared)
        .get("/task/list", task_list)
        .post("/task/start", task_start)
        .post("/task/stop", task_stop)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

/// Routerify app state, include bridger common config
fn app_state(options: ServerOptions) -> anyhow::Result<BridgeState> {
    let base_path = patch::bridger::base_path(options.base_path)?;
    Ok(BridgeState {
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

/// Start shared service
async fn start_shared(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let saved_shared = dc::get_shared();
    if saved_shared.is_some() {
        return Resp::<String>::ok_with_msg("Shared already started").response_json();
    }

    let param: SharedStartParam = patch::hyper::deserialize_body(&mut req).await?;
    let state = req.data::<BridgeState>().unwrap();

    let path_config = state
        .base_path
        .join(format!("{}.{}", SharedTask::NAME, param.format));
    if let Some(config_raw) = param.config {
        tokio::fs::write(&path_config, &config_raw).await?
    }
    if !path_config.exists() {
        return Resp::<String>::err_with_msg(format!(
            "The config file not found: {:?}",
            path_config
        ))
        .response_json();
    }

    let mut c = config::Config::default();
    c.merge(config::File::from(path_config))?;
    let shared_config = c
        .try_into::<SharedConfig>()
        .map_err(|e| StandardError::Api(format!("Failed to load shared config: {:?}", e)))?;
    let shared = BridgeShared::new(shared_config)?;
    dc::set_shared(shared)?;

    Resp::<String>::ok().response_json()
}

/// Get task list
async fn task_list(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let tasks = dc::available_tasks()?;
    let data = tasks
        .iter()
        .map(|item| {
            let running = dc::task_is_running(item).unwrap_or(false);
            TaskListResponse {
                name: item.clone(),
                running,
            }
        })
        .collect::<Vec<TaskListResponse>>();
    Resp::ok_with_data(data).response_json()
}

/// Start a task
async fn task_start(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStartParam = patch::hyper::deserialize_body(&mut req).await?;
    if dc::task_is_running(&param.name)? {
        return Resp::<String>::ok_with_msg(format!("The task [{}] is running", &param.name))
            .response_json();
    }

    let state = req.data::<BridgeState>().unwrap();

    let shared = dc::get_shared().ok_or_else(|| {
        StandardError::Api("The shared service isn't started, please start it first.".to_string())
    })?;
    let channel = shared.channel();

    match &param.name[..] {
        DarwiniaEthereumTask::NAME => {
            let path_config =
                state
                    .base_path
                    .join(format!("{}.{}", DarwiniaEthereumTask::NAME, param.format));
            if let Some(config_raw) = param.config {
                tokio::fs::write(&path_config, &config_raw).await?
            }
            if !path_config.exists() {
                return Resp::<String>::err_with_msg(format!(
                    "The config file not found: {:?}",
                    path_config
                ))
                .response_json();
            }
            let mut c = config::Config::default();
            c.merge(config::File::from(path_config))?;
            let task_config = c
                .try_into::<DarwiniaEthereumConfig>()
                .map_err(|e| StandardError::Api(format!("Failed to load task config: {:?}", e)))?;
            let task = DarwiniaEthereumTask::new(task_config, channel.clone()).await?;
            dc::keep_task(DarwiniaEthereumTask::NAME, Box::new(task))?;
        }
        PangolinMillauTask::NAME => {
            let path_config =
                state
                    .base_path
                    .join(format!("{}.{}", PangolinMillauTask::NAME, param.format));
            if let Some(config_raw) = param.config {
                tokio::fs::write(&path_config, &config_raw).await?
            }
            if !path_config.exists() {
                return Resp::<String>::err_with_msg(format!(
                    "The config file not found: {:?}",
                    path_config
                ))
                .response_json();
            }
            let mut c = config::Config::default();
            c.merge(config::File::from(path_config))?;
            let task_config = c
                .try_into::<PangolinMillauConfig>()
                .map_err(|e| StandardError::Api(format!("Failed to load task config: {:?}", e)))?;
            let task = PangolinMillauTask::new(task_config, channel.clone()).await?;
            dc::keep_task(PangolinMillauTask::NAME, Box::new(task))?;
        }
        _ => {
            return Resp::<String>::err_with_msg(format!("Not support this task [{}]", &param.name))
                .response_json()
        }
    }

    Resp::<String>::ok().response_json()
}

/// Start a task
async fn task_stop(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStopParam = patch::hyper::deserialize_body(&mut req).await?;
    log::debug!("{:?}", param);
    let task_name = param.name;
    dc::stop_task(&task_name)?;
    log::warn!("The task {} is stopped", task_name);
    Resp::<String>::ok().response_json()
}
