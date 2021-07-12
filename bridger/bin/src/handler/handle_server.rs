use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::error::StandardError;
use component_state::config::{BridgeStateConfig, MicrokvConfig};
use component_state::state::BridgeStateComponent;

use crate::handler::task_manager;
use crate::patch;
use crate::types::command::ServerOptions;
use crate::types::server::{Resp, WebserverState};
use crate::types::transfer::{
    TaskConfigTemplateParam, TaskListResponse, TaskStartParam, TaskStopParam,
};

/// Handler bridger server
pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
    init(options.clone()).await?;
    auto_start_task(options.clone()).await?;
    start_webserver(options).await?;
    Ok(())
}

async fn init(options: ServerOptions) -> anyhow::Result<()> {
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
    Ok(())
}

async fn auto_start_task(options: ServerOptions) -> anyhow::Result<()> {
    let base_path = patch::bridger::base_path(options.base_path)?;
    crate::handler::task_manager::auto_start_task(base_path).await
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
        .post("/task/config-template", task_config_template)
        .post("/task/:task_name/:task_route", task_route)
        .any(handler_404)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

/// Routerify app state, include bridger common config
async fn app_state(options: ServerOptions) -> anyhow::Result<WebserverState> {
    let base_path = patch::bridger::base_path(options.base_path)?;

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

async fn handler_404(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let uri = req.uri();
    Resp::<String>::err_with_msg(format!("Not found this api: {}", uri))
        .response_json_with_code(hyper::StatusCode::NOT_FOUND)
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

/// Start a task
async fn task_start(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskStartParam = patch::hyper::deserialize_body(&mut req).await?;

    let state_webserver = req.data::<WebserverState>().unwrap();
    let base_path = &state_webserver.base_path.as_ref();
    if let Err(e) = task_manager::start_task_single(base_path.into(), param).await {
        return Resp::<String>::err_with_msg(format!("{}", e)).response_json();
    }
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

async fn task_config_template(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: TaskConfigTemplateParam = patch::hyper::deserialize_body(&mut req).await?;
    let config_template = task_manager::task_config_template(param)?;
    Resp::ok_with_data(config_template).response_json()
}
