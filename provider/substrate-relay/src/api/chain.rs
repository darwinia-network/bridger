use std::sync::Mutex;

use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::persist::{Chain, Persist};
use crate::types::patch::resp::Resp;

#[post("/api/chain/add")]
pub async fn chain_add(
	data_persist: web::Data<Mutex<Persist>>,
	req: HttpRequest,
	form: web::Form<Chain>,
) -> Result<HttpResponse, crate::error::WebError> {
	let mut persist = data_persist.lock().unwrap();
	persist.chain_add(form.0).await?;
	let chains = persist.chains();
	Ok(HttpResponse::Ok().json(Resp::ok_with_data(chains)))
}
