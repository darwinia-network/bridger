use crate::types::patch::resp::Resp;
use actix_web::body::{Body, ResponseBody};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http};

pub fn render_error<B>(
	mut res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
	res.response_mut().headers_mut().insert(
		http::header::CONTENT_TYPE,
		http::HeaderValue::from_static("application/json"),
	);
	let body = Resp::<String>::err_with_msg("response code not 200");
	let new_res =
		res.map_body(|_head, _body| ResponseBody::Other(Body::Message(Box::new(body.to_json()))));
	Ok(ErrorHandlerResponse::Response(new_res))
}
