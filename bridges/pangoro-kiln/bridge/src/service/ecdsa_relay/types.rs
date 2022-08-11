use client_pangoro::client::PangoroClient;
use subquery::Subquery;

#[derive(Clone)]
pub struct EcdsaSource {
    pub subquery: Subquery,
    pub client: PangoroClient,
}
