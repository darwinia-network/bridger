use bee_client::api::{Api, Event};
use bee_client::types::client::ChainTypes;

pub type SubstrateApi<T: ChainTypes> = bee_client::ws::BeeWebsocket<T>;
