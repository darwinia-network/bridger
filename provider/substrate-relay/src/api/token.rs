use actix_web::{post, HttpRequest, HttpResponse, Responder};

use crate::types::patch::resp::Resp;

#[post("/api/token/generate")]
pub async fn generate(req: HttpRequest) -> impl Responder {
	HttpResponse::Ok().json(Resp::ok_with_data("test"))
}
