use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_traits::bridge::component::BridgeComponent;
use component_state::config::{BridgeStateConfig, MicrokvConfig};
use component_state::state::BridgeStateComponent;
use support_keep::types::WebserverState;

use crate::patch;
use crate::route;
use crate::route::task_manager;
use crate::types::command::ServerOptions;
use crate::types::server::Resp;

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
    support_keep::state::set_state_bridge(bridge_state)?;
    Ok(())
}

async fn auto_start_task(options: ServerOptions) -> anyhow::Result<()> {
    let base_path = patch::bridger::base_path(options.base_path)?;
    task_manager::auto_start_task(base_path).await
}

async fn start_webserver(options: ServerOptions) -> anyhow::Result<()> {
    let router = router(options.clone()).await;

    let service = RouterService::new(router).unwrap();

    let host_port = format!("{}:{}", options.host, options.port);
    let addr: SocketAddr = host_port.parse()?;

    let server = Server::bind(&addr).serve(service);

    log::info!("bridger is running on: {}", addr);
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
        .get("/task/list", route::task::task_list)
        .post("/task/start", route::task::task_start)
        .post("/task/stop", route::task::task_stop)
        .post("/task/config-template", route::task::task_config_template)
        .post("/task/:task_name/:task_route", route::task::task_route)
        .post("/kv/put", route::kv::put)
        .post("/kv/get", route::kv::get)
        .post("/kv/list", route::kv::list)
        .post("/kv/remove", route::kv::remove)
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
