use std::sync::Mutex;

use actix_web::{post, web, HttpResponse};

use crate::error;
use crate::persist::{Chain, Persist};
use crate::types::cond::relay::InitBridgeCond;
use crate::types::patch::resp::Resp;

#[post("/api/relay/init-bridge")]
pub async fn init_bridge(
    data_persist: web::Data<Mutex<Persist>>,
    form: web::Form<InitBridgeCond>,
) -> Result<HttpResponse, crate::error::WebError> {
    let persist = data_persist.lock().unwrap();
    let source_name: &String = form.0.source();
    let target_name: &String = form.0.target();
    let chains: &Vec<Chain> = persist.chains();
    let source_chain = chains
        .iter()
        .find(|&item| item.name() == source_name)
        .ok_or(error::CliError::ChainNotFound)?;
    let target_chain = chains
        .iter()
        .find(|&item| item.name() == target_name)
        .ok_or(error::CliError::ChainNotFound)?;
    // let source = source_chain.to_substrate_relay_chain::<Source>().await?;

    debug!("{:?}", form);
    Ok(HttpResponse::Ok().json(Resp::ok_with_data("")))
}
