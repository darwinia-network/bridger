use thiserror::Error as ThisError;

pub type Result<T> = anyhow::Result<T>;

use crate::types::patch::resp::Resp;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display as DeviceMoreDisplay, Error as DeriveMoreError};

#[derive(ThisError, Debug)]
pub enum CliError {
	#[error("The config file format isn't TOML")]
	ConfigPathNotToml,
	#[error("The config path is not a file")]
	ConfigPathNotFile,
	#[error("This chain name is exists")]
	ChainNameExists,
}

#[derive(Debug, DeviceMoreDisplay, DeriveMoreError)]
#[display(fmt = "{}", message)]
pub struct WebError {
	message: String,
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
