use std::path::PathBuf;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Resp<T: Serialize> {
    err: u8,
    msg: String,
    trace: Option<String>,
    data: Option<T>,
}

impl<T: Serialize + DeserializeOwned> Resp<T> {
    pub fn ok(data: T) -> Self {
        Self {
            err: 0,
            msg: "success".to_string(),
            trace: None,
            data: Some(data),
        }
    }
    pub fn err<M: AsRef<str>>(msg: M, data: Option<T>) -> Self {
        Self {
            err: 1,
            msg: msg.as_ref().to_string(),
            trace: None,
            data,
        }
    }
    pub fn err_with_trace<M: AsRef<str>, R: AsRef<str>>(msg: M, trace: R, data: Option<T>) -> Self {
        Self {
            err: 1,
            msg: msg.as_ref().to_string(),
            trace: Some(trace.as_ref().to_string()),
            data,
        }
    }
}

impl<T: Serialize + DeserializeOwned> Resp<T> {
    pub fn response_json(&self) -> anyhow::Result<tide::Response> {
        let code = if self.err == 1 {
            tide::StatusCode::Ok
        } else {
            tide::StatusCode::BadRequest
        };
        let value = serde_json::to_value(self)?;
        Ok(tide::Response::builder(code)
            .header("Content-Type", "application/json")
            .body(value)
            .build())
    }
}

// -- bridge state

#[derive(Clone)]
pub struct BridgeState {
    pub base_path: Arc<PathBuf>,
}
