use client_pangoro::client::PangoroClient;
use subquery::Subquery;

#[derive(Clone)]
pub struct EcdsaSource {
    pub block: Option<u32>,
    pub subquery: Subquery,
    pub client: PangoroClient,
    pub ecdsa_private_key: [u8; 20],
}
