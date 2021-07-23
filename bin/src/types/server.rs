#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "T: Serialize, for<'a> T: Deserialize<'a>")]
pub struct Resp<T: Serialize + for<'a> Deserialize<'a>> {
    err: u8,
    msg: String,
    trace: Option<String>,
    data: Option<T>,
}

impl<T: Serialize + for<'a> Deserialize<'a>> Resp<T> {
    pub fn ok() -> Self {
        Self::ok_with_msg("success")
    }
    pub fn ok_with_msg<M: AsRef<str>>(msg: M) -> Self {
        Self {
            err: 0,
            msg: msg.as_ref().to_string(),
            trace: None,
            data: None,
        }
    }
    pub fn ok_with_data(data: T) -> Self {
        Self::ok_with_msg_and_data("success", data)
    }
    pub fn ok_with_msg_and_data<M: AsRef<str>>(msg: M, data: T) -> Self {
        Self {
            err: 0,
            msg: msg.as_ref().to_string(),
            trace: None,
            data: Some(data),
        }
    }
    pub fn err_with_msg<M: AsRef<str>>(msg: M) -> Self {
        Self {
            err: 1,
            msg: msg.as_ref().to_string(),
            trace: None,
            data: None,
        }
    }
    pub fn err_with_msg_and_data<M: AsRef<str>>(msg: M, data: T) -> Self {
        Self {
            err: 1,
            msg: msg.as_ref().to_string(),
            trace: None,
            data: Some(data),
        }
    }
    pub fn err_with_trace<M: AsRef<str>, R: AsRef<str>>(msg: M, trace: R) -> Self {
        Self {
            err: 1,
            msg: msg.as_ref().to_string(),
            trace: Some(trace.as_ref().to_string()),
            data: None,
        }
    }
}

impl<T: Serialize + for<'a> Deserialize<'a>> Resp<T> {
    pub fn msg(&self) -> &String {
        &self.msg
    }
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    pub fn trace(&self) -> Option<&String> {
        self.trace.as_ref()
    }
    pub fn is_ok(&self) -> bool {
        self.err == 0
    }
    pub fn is_err(&self) -> bool {
        self.err == 1
    }
    fn _response_json_with_code(
        &self,
        code: Option<hyper::StatusCode>,
    ) -> anyhow::Result<hyper::Response<hyper::Body>> {
        let xcode = if let Some(code) = code {
            code
        } else if self.err == 0 {
            hyper::StatusCode::OK
        } else {
            hyper::StatusCode::BAD_REQUEST
        };
        let value = serde_json::to_string(self)?;
        let response = hyper::Response::builder()
            .status(xcode)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(value))?;
        Ok(response)
    }
    pub fn response_json_with_code(
        &self,
        code: hyper::StatusCode,
    ) -> anyhow::Result<hyper::Response<hyper::Body>> {
        self._response_json_with_code(Some(code))
    }
    pub fn response_json(&self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        self._response_json_with_code(None)
    }
}
