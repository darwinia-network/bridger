use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

use bridge_shared::shared::{BridgeShared, SharedConfig, SharedTask};
use bridge_standard::bridge::task::BridgeSand;
use bridge_standard::error::StandardError;

use crate::types::command::ServerOptions;
use crate::types::server::{BridgeState, Resp};
use crate::types::transfer::SharedStartParam;
use crate::{dc, patch};

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
    log::error!("{:?}", err);
    let msg = format!("{}", err);
    Resp::<String>::err_with_trace(msg, "", None)
        .response_json()
        .expect("Failed to build response")
}

async fn hello(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    Ok(Resp::<String>::ok().response_json()?)
}

async fn start_shared(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let saved_shared = dc::get_shared();
    if saved_shared.is_some() {
        return Resp::<String>::ok_with_msg("Shared already started").response_json();
    }

    let param: SharedStartParam = patch::hyper::deserialize_body(&mut req).await?;
    let state = req.data::<BridgeState>().unwrap();

    let path_config = state.base_path.join(format!("{}.toml", SharedTask::NAME));
    if let Some(config_raw) = param.config {
        tokio::fs::write(&path_config, &config_raw).await?
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

    Resp::<String>::ok().response_json()
}

async fn task_list(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let tasks = dc::available_tasks()?;
    Resp::ok_with_data(tasks).response_json()
}
