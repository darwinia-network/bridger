use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_shared::shared::{BridgeShared, SharedConfig, SharedTask};
use bridge_standard::bridge::task::BridgeSand;
use bridge_standard::error::StandardError;

use crate::dc;
use crate::types::command::ServerOptions;
use crate::types::server::{BridgeState, Resp};
use crate::types::transfer::SharedStartParam;

pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
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

fn router(options: ServerOptions) -> Router<Body, anyhow::Error> {
    let state = app_state(options).expect("Failed to build app state");
    Router::builder()
        .data(state)
        .middleware(Middleware::pre(logger))
        .get("/", hello)
        .post("/shared/start", start_shared)
        .get("/task/list", task_list)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

fn app_state(options: ServerOptions) -> anyhow::Result<BridgeState> {
    let base_path = options.base_path.unwrap_or_else(|| {
        let mut path = std::env::temp_dir();
        path.push("darwinia-bridger");
        path
    });
    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    Ok(BridgeState {
        base_path: Arc::new(base_path),
    })
}

async fn logger(req: Request<Body>) -> anyhow::Result<Request<Body>> {
    log::debug!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    let msg = format!("Something went wrong: {}", err);
    Resp::<String>::err_with_trace(msg, "", None)
        .response_json()
        .expect("Failed to build response")
}

async fn hello(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    Ok(Resp::ok("hello".to_string()).response_json()?)
}

async fn start_shared(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let saved_shared = dc::get_shared();
    if saved_shared.is_some() {
        return Ok(Resp::ok("Shared already started".to_string()).response_json()?);
    }

    let state = req.data::<BridgeState>().unwrap();
    let file_name = format!("{}.toml", SharedTask::NAME);
    let path_config = state.base_path.join(file_name);
    let is_upload = req
        .param("is_upload")
        .map(|item| item == "true" || item == "1")
        .unwrap_or(false);
    if is_upload {
        let config_raw = req
            .param("config")
            .ok_or_else(|| StandardError::Api("The config content is required".to_string()))?;
        tokio::fs::write(&path_config, config_raw).await?
    }
    if !path_config.exists() {
        return Ok(Resp::<String>::err(
            format!("The config file not found: {:?}", path_config),
            None,
        )
        .response_json()?);
    }

    let mut c = config::Config::default();
    c.merge(config::File::from(path_config))?;
    let shared_config = c
        .try_into::<SharedConfig>()
        .map_err(|e| StandardError::Api(format!("Failed to load shared config: {:?}", e)))?;
    let shared = BridgeShared::new(shared_config)?;
    dc::set_shared(shared)?;

    Ok(Resp::ok("success".to_string()).response_json()?)
}

async fn task_list(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let tasks = dc::available_tasks()?;
    Ok(Resp::ok(tasks).response_json()?)
}
