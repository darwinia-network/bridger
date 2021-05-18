use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display as DeviceMoreDisplay, Error as DeriveMoreError};
use thiserror::Error as ThisError;

use crate::types::patch::resp::Resp;

pub type Result<T> = anyhow::Result<T>;

#[derive(ThisError, Debug)]
pub enum CliError {
	#[error("The config file format isn't TOML")]
	ConfigPathNotToml,
	#[error("The config path is not a file")]
	ConfigPathNotFile,
	#[error("This chain name is exists")]
	ChainNameExists,
	#[error("Not found this chain: [{0}]")]
	ChainNotFound(String),
	#[error("Remove chain error")]
	ChainRemoveError,
	#[error("Not found this token")]
	TokenNotFound,
	#[error("Request error: {0}")]
	RequestError(String),
	#[error("Api error: [{0}] {1}")]
	ApiError(String, String),
	#[error("Not support this chain {0}")]
	NotSupportChain(String),
	#[error("Not support bridge {0} -> {1}")]
	NotSupportBridge(String, String),
	#[error("The lane id is error")]
	LaneIdError,
}

#[derive(Debug, DeviceMoreDisplay, DeriveMoreError)]
#[display(fmt = "{}", message)]
pub struct WebError {
	message: String,
}

impl WebError {
	pub fn new<S: AsRef<str>>(message: S) -> Self {
		Self {
			message: message.as_ref().to_string(),
		}
	}
}

impl From<CliError> for WebError {
	fn from(error: CliError) -> Self {
		let message = error.to_string();
		Self { message }
	}
}

impl From<anyhow::Error> for WebError {
	fn from(error: anyhow::Error) -> Self {
		let message = error.to_string();
		Self { message }
	}
}

impl ResponseError for WebError {
	fn error_response(&self) -> HttpResponse {
		HttpResponse::Ok().json(Resp::<String>::err_with_msg(&self.message))
	}
}
