use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse};

use crate::persist::Persist;
use crate::types::cond::token::{TokenGenerateCond, TokenRemoveCond};
use crate::types::patch::resp::Resp;

#[post("/api/token/generate")]
pub async fn generate(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<TokenGenerateCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	let token = persist.token_generate(form.0.remark().clone()).await?;
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(token)))
}

#[get("/api/token/list")]
pub async fn list(
	data_persist: web::Data<Mutex<Persist>>,
) -> Result<HttpResponse, crate::error::WebError> {
	let persist = data_persist.lock().unwrap();
	let tokens = persist.token_list().await;
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(tokens)))
}

#[post("/api/token/remove")]
pub async fn remove(
	data_persist: web::Data<Mutex<Persist>>,
	form: web::Form<TokenRemoveCond>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	persist.token_remove(form.0.token()).await?;
	Ok(HttpResponse::Ok().json(Resp::<String>::ok()))
}
