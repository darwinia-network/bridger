use std::sync::Mutex;

use actix_web::{post, web, HttpResponse};

use crate::error;
use crate::persist::{Chain, Persist};
use crate::types::cond::relay::{SourceAndTargetCond, StartRelayCond};
use crate::types::patch::resp::Resp;
use relay_chain::types::transfer::RelayHeadersAndMessagesInfo;

macro_rules! init_bridge {
	($chain_name:expr, $generic:tt) => {};
}

#[post("/api/relay/init-bridge")]
pub async fn init_bridge(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<SourceAndTargetCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let persist = data_persist.lock().unwrap();
	let source_name: &String = form.0.source();
	let target_name: &String = form.0.target();

	let source_chain = persist
		.find_chain(source_name)
		.ok_or(error::CliError::ChainNotFound(source_name.to_string()))?;
	let target_chain = persist
		.find_chain(target_name)
		.ok_or(error::CliError::ChainNotFound(target_name.to_string()))?;

	relay_chain::s2s::init_bridge::run(source_chain.to_chain_info(), target_chain.to_chain_info()).await?;
	Ok(HttpResponse::Ok().json(Resp::ok_with_data("")))
}

#[post("/api/relay/start")]
pub async fn start(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<StartRelayCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let persist = data_persist.lock().unwrap();
	let cond = form.0;
	let source_name: &String = cond.source();
	let target_name: &String = cond.target();

	let source_chain = persist
		.find_chain(source_name)
		.ok_or(error::CliError::ChainNotFound(source_name.to_string()))?;
	let target_chain = persist
		.find_chain(target_name)
		.ok_or(error::CliError::ChainNotFound(target_name.to_string()))?;

	let mut relay_info = RelayHeadersAndMessagesInfo::default();
	relay_info.set_source(source_chain.to_chain_info());
	relay_info.set_target(target_chain.to_chain_info());
	relay_info.set_lanes(cond.lanes().clone());

	let prometheus_info = cond.prometheus_info();
	relay_info.set_prometheus_params(prometheus_info);

	relay_chain::s2s::relay_headers_and_messages::run(relay_info).await?;
	Ok(HttpResponse::Ok().json(Resp::ok_with_data("")))
}
