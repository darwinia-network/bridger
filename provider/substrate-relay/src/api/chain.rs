use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse};

use crate::persist::{Chain, Persist};
use crate::types::cond::chain::ChainRemoveCond;
use crate::types::patch::resp::Resp;

#[get("/api/chain/list")]
pub async fn chain_list(
	data_persist: web::Data<Mutex<Persist>>,
) -> Result<HttpResponse, crate::error::WebError> {
	let persist = data_persist.lock().unwrap();
	let chains = persist.chain_list().await;
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(chains)))
}

#[post("/api/chain/add")]
pub async fn chain_add(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<Chain>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	persist.chain_add(form.0).await?;
	let chains = persist.chains();
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(chains)))
}

#[post("/api/chain/update")]
pub async fn chain_update(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<Chain>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	persist.chain_update(form.0).await?;
	let chains = persist.chains();
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(chains)))
}

#[post("/api/chain/remove")]
pub async fn chain_remove(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<ChainRemoveCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	persist.chain_remove(form.0.name()).await?;
	let chains = persist.chains();
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(chains)))
}
