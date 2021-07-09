use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use bridge_traits::bridge::task::TaskRouterAsyncFn;
use bridge_traits::error::StandardError;

static CUSTOM_TASK_ROUTER: Lazy<Mutex<HashMap<String, TaskRouterAsyncFn>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn merge_route(router_new: HashMap<String, TaskRouterAsyncFn>) -> anyhow::Result<()> {
    let mut router_keep = CUSTOM_TASK_ROUTER
        .lock()
        .map_err(|_e| StandardError::Api("failed to get custom task router".to_string()))?;
    for (key, value) in router_new.into_iter() {
        router_keep.insert(key, value);
    }
    Ok(())
}

pub async fn run_route<U: AsRef<str>>(
    uri: U,
    param: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let uri = uri.as_ref();
    let v = futures::executor::block_on(async {
        let router_keep = CUSTOM_TASK_ROUTER
            .lock()
            .map_err(|_e| StandardError::Api("failed to get custom task router".to_string()))?;
        if let Some(route) = router_keep.get(uri) {
            return (route)(param).await;
        }
        Err(StandardError::Api(format!("Not found this task router: [{}]", uri)).into())
    });
    v
}
