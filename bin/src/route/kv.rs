use hyper::{Body, Request, Response};
use microkv::namespace::NamespaceMicroKV;

use crate::patch;
use crate::types::server::Resp;
use crate::types::transfer::{KvListParam, KvOperationParam};

fn microkv(namespace: Option<String>) -> anyhow::Result<NamespaceMicroKV> {
    let state = support_keep::state::get_state_bridge_ok()?;
    Ok(state.microkv_with_namespace(namespace.unwrap_or_else(|| "".to_string())))
}

pub async fn ns(_req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let state = support_keep::state::get_state_bridge_ok()?;
    let microkv = state.microkv();
    let ns = microkv.namespaces()?;
    Resp::ok_with_data(ns).response_json()
}

pub async fn put(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: KvOperationParam = patch::hyper::deserialize_body(&mut req).await?;
    let keys = param.keys;
    let values = param.values;
    if keys.len() != values.len() {
        return Resp::<String>::err_with_msg("The length not same by keys and values")
            .response_json();
    }
    if keys.is_empty() {
        return Resp::<String>::err_with_msg("The keys and values is required").response_json();
    }
    let microkv = microkv(param.namespace)?;
    let len = keys.len();
    for i in 0..len {
        let key_input = keys.get(i).expect("unreachable");
        let value_input = values.get(i).expect("unreachable");
        if key_input.is_empty() {
            continue;
        }
        patch::helpers::spec_serialize_value(&microkv, key_input.clone(), value_input.clone())?;
    }
    Resp::<String>::ok().response_json()
}

pub async fn get(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: KvOperationParam = patch::hyper::deserialize_body(&mut req).await?;
    let keys = param.keys;
    if keys.is_empty() {
        return Resp::<String>::err_with_msg("The keys is required").response_json();
    }
    let microkv = microkv(param.namespace)?;
    let mut values = vec![];
    for key in keys {
        let value = microkv.get(key)?;
        values.push(value);
    }
    Resp::ok_with_data(values).response_json()
}

pub async fn list(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: KvListParam = patch::hyper::deserialize_body(&mut req).await?;
    let sorted = param.sorted;
    let microkv = microkv(param.namespace)?;
    let keys = if sorted {
        microkv.sorted_keys()?
    } else {
        microkv.keys()?
    };
    Resp::ok_with_data(keys).response_json()
}

pub async fn remove(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let param: KvOperationParam = patch::hyper::deserialize_body(&mut req).await?;
    let keys = param.keys;
    if keys.is_empty() {
        return Resp::<String>::err_with_msg("The keys is required").response_json();
    }
    let microkv = microkv(param.namespace)?;
    for key in keys {
        microkv.delete(key)?;
    }
    Resp::<String>::ok().response_json()
}
