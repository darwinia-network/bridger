use std::sync::Arc;

use tide::prelude::*;
use tide::{Body, Request, Response, StatusCode};

use bridge_shared::shared::SharedTask;
use bridge_standard::bridge::task::BridgeSand;

use crate::dc;
use crate::types::command::ServerOptions;
use crate::types::tide::{BridgeState, Resp};
use tide::utils::After;

pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
    let state = app_state(options.clone())?;

    let mut app = tide::with_state(state);

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.error() {
            let err_msg = format!("{:?}", err);
            let mut split = err_msg.split("\n");
            let msgs: Vec<&str> = split.collect();
            let mut iter = msgs.into_iter();
            let msg = iter.nth(0).unwrap_or_else(|| "Unknown error");
            let trace = iter.collect::<Vec<&str>>().join("\n");
            log::error!("{} \n{}", msg, trace);
            res.set_status(StatusCode::InternalServerError);

            let resp = Resp::<String>::err_with_trace(msg, trace, None);
            res.set_body(Body::from_json(&resp)?);
        }
        Ok(res)
    }));

    app.at("/").get(hello);
    app.at("/shared/start").post(start_shared);
    app.at("/task/list").get(task_list);

    let addr = format!("{}:{}", options.host, options.port);
    println!("Server listening on http://{}", addr);
    app.listen(&addr).await?;
    Ok(())
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

async fn hello(_req: Request<BridgeState>) -> tide::Result {
    Ok(Resp::ok("hello".to_string()).response_json()?)
}

async fn start_shared(mut req: Request<BridgeState>) -> tide::Result {
    let state = req.state();
    let file_name = format!("{}.toml", SharedTask::NAME);
    let path_config = state.base_path.join(file_name);
    let is_upload = req.param("is_upload")?;
    if is_upload == "true" || is_upload == "1" {
        let config_raw = req.param("config")?;
        tokio::fs::write(&path_config, config_raw).await?;
    }
    if !path_config.exists() {
        return Ok(Resp::<String>::err(
            format!("The config file not found: {:?}", path_config),
            None,
        )
        .response_json()?);
    }
    let config = tokio::fs::read_to_string(&path_config).await?;

    Ok(Resp::ok(config).response_json()?)
}

async fn task_list(_req: Request<BridgeState>) -> tide::Result {
    let tasks = dc::available_tasks()?;
    Ok(Resp::ok(tasks).response_json()?)
}
