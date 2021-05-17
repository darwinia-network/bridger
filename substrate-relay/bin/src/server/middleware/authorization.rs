use std::sync::Mutex;
use std::task::{Context, Poll};

use actix_web::{dev::Service, dev::ServiceRequest, dev::ServiceResponse, dev::Transform, web, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

use crate::persist::{Generic, Persist, Token};
use crate::types::patch::resp::Resp;

pub struct Authorization;

impl<S, B> Transform<S> for Authorization
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Transform = AuthorizationMiddleware<S>;
	type InitError = ();
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(AuthorizationMiddleware { service })
	}
}

pub struct AuthorizationMiddleware<S> {
	service: S,
}

impl<S, B> Service for AuthorizationMiddleware<S>
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

	fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		let uri = req.path();

		if !uri.starts_with("/api") || uri == "/" {
			return Either::Left(self.service.call(req));
		}

		let data_persist = req.app_data::<web::Data<Mutex<Persist>>>().unwrap().clone();
		let persist = data_persist.lock().unwrap();

		let generic: &Generic = persist.generic();
		// if not enable authorization, pass direct.
		if !generic.enable_auth() {
			return Either::Left(self.service.call(req));
		}

		let allow_tokens: &Vec<Token> = persist.tokens();

		let header_token = req
			.headers()
			.iter()
			.find(|item| item.0 == "Authorization" || item.0 == "token");

		let token_request = match header_token {
			Some(header) => {
				let header_token_value = header.1;
				match header_token_value.to_str() {
					Ok(v) => v.replace("Bearer", "").replace(" ", ""),
					Err(_e) => {
						return Either::Right(ok(req.into_response(
							HttpResponse::Forbidden()
								.json(Resp::<String>::err_with_msg("Token value is not allow"))
								.into_body(),
						)));
					}
				}
			}
			None => {
				return Either::Right(ok(req.into_response(
					HttpResponse::Forbidden()
						.json(Resp::<String>::err_with_msg("Not found token in request"))
						.into_body(),
				)));
			}
		};

		// is allow
		if !allow_tokens.iter().any(|item| item.value() == &token_request) {
			return Either::Right(ok(req.into_response(
				HttpResponse::Forbidden()
					.json(Resp::<String>::err_with_msg("Authorization failed"))
					.into_body(),
			)));
		}

		Either::Left(self.service.call(req))
	}
}
