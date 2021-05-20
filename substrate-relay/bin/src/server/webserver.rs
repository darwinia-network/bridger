use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, middleware, web, App, HttpServer};
use getset::{Getters, Setters};
use typed_builder::TypedBuilder;

use crate::error::Result;
use crate::persist::{Generic, Persist};
use crate::server::api;

#[derive(Debug, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct WebServer {
	persist: Persist,
}

impl WebServer {
	pub async fn run(&self) -> Result<()> {
		let generic: &Generic = self.persist.generic();
		let addr = format!("{}:{}", generic.host(), generic.port());
		let persist_data = web::Data::new(Mutex::new(self.persist.clone()));
		info!("Listen: {}", addr);
		Ok(HttpServer::new(move || {
			App::new()
				.app_data(persist_data.clone())
				// enable logger
				.wrap(middleware::Logger::default())
				.wrap(Cors::permissive())
				.wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, api::generic::render_error))
				.wrap(crate::server::middleware::authorization::Authorization)
				// register simple handler
				.service(web::resource("/").to(|| async { "Hello!" }))
				// .service(api::chain::chain_list)
				// .service(api::chain::chain_add)
				// .service(api::chain::chain_update)
				// .service(api::chain::chain_remove)
				// .service(api::token::generate)
				// .service(api::token::list)
				// .service(api::token::remove)
				.service(api::relay::init_bridge)
				.service(api::relay::start)
				.service(api::relay::stop)
				.service(api::relay::status)
		})
		.bind(addr)?
		.run()
		.await?)
	}
}
