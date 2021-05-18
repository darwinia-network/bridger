use actix_web::{get, post, web, HttpResponse};
use chain_relay::types::transfer::{HexLaneId, RelayHeadersAndMessagesInfo};
use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use crate::error;
use crate::persist::Persist;
use crate::types::cond::relay::{SourceAndTargetCond, StartRelayCond, StatusRelayCond, StopRelayCond};
use crate::types::patch::resp::Resp;

static THREAD_CACHE: Lazy<Mutex<HashMap<String, JoinHandle<()>>>> = Lazy::new(|| {
	let mut m = HashMap::new();
	Mutex::new(m)
});

fn bridge_name(source: &String, target: &String) -> String {
	format!("{}_{}", source, target)
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

	chain_relay::s2s::init_bridge::run(source_chain.to_chain_info(), target_chain.to_chain_info()).await?;
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

	// fixme: there need support multiple lanes need use Vec<String>
	// let lanes_string: Vec<String> = cond.lanes().clone();
	// let lanes_result = lanes_string
	// 	.iter()
	// 	.map(|item| HexLaneId::from_str(&item[..]))
	// 	.collect::<Vec<Result<HexLaneId, _>>>();
	// let mut lanes = Vec::with_capacity(lanes_result.len());
	// for item in lanes_result {
	// 	let lane = match item {
	// 		Ok(v) => Ok(v),
	// 		Err(_) => Err(crate::error::CliError::LaneIdError),
	// 	}?;
	// 	lanes.push(lane);
	// }

	let lane = match HexLaneId::from_str(&cond.lanes()[..]) {
		Ok(v) => Ok(v),
		Err(_) => Err(crate::error::CliError::LaneIdError),
	}?;
	let lanes = vec![lane];

	relay_info.set_lanes(lanes);

	let prometheus_info = cond.prometheus_info();
	relay_info.set_prometheus_params(prometheus_info);

	let bridge_name = bridge_name(source_name, target_name);

	let mut tc = THREAD_CACHE.lock().unwrap();

	if tc.contains_key(&bridge_name) {
		return Err(crate::error::WebError::new("This bridge is already started"));
	}

	let thread_control: JoinHandle<()> = thread::spawn(move || {
		// fixme: because futures::executor::block_on will block thread, so mpsc::channel not work in here, can not receive data to park this thread
		// thread::park();
		futures::executor::block_on(async {
			chain_relay::s2s::relay_headers_and_messages::run(relay_info.clone())
				.await
				.unwrap();
		});
	});

	tc.insert(bridge_name, thread_control);
	Ok(HttpResponse::Ok().json(Resp::ok_with_data("")))
}

#[post("/api/relay/stop")]
pub async fn stop(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<StopRelayCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let cond = form.0;
	let source_name: &String = cond.source();
	let target_name: &String = cond.target();
	let bridge_name = bridge_name(source_name, target_name);

	let tc = THREAD_CACHE.lock().unwrap();
	let _thread_saved = tc
		.get(&bridge_name)
		.ok_or(error::WebError::new("This bridge is not start"))?;
	// thread_saved.thread().unpark();
	// Ok(HttpResponse::Ok().json(Resp::ok_with_data("Running")))
	Err(error::WebError::new("Not support now"))
}

#[get("/api/relay/status")]
pub async fn status(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<StatusRelayCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let cond = form.0;
	let source_name: &String = cond.source();
	let target_name: &String = cond.target();
	let bridge_name = bridge_name(source_name, target_name);

	let tc = THREAD_CACHE.lock().unwrap();
	match tc.get(&bridge_name) {
		Some(_) => Ok(HttpResponse::Ok().json(Resp::ok_with_data("Running"))),
		None => Ok(HttpResponse::Ok().json(Resp::ok_with_data("Stop"))),
	}
}
