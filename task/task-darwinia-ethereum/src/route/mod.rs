use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::DarwiniaEthereumBus;

mod affirm;
mod confirm;
mod ecdsa;
mod guard;
mod info;
mod keys;
mod mmr;
mod parcel;
mod relay;
mod starter;

pub async fn dispatch_route(
    bus: &DarwiniaEthereumBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay::route(bus, param).await,
        "start-darwinia" => starter::start_darwinia(bus, param).await,
        "start-ethereum" => starter::start_ethereum(bus, param).await,
        "show-parcel" => parcel::show(bus, param).await,
        "affirm" => affirm::affirm(bus, param).await,
        "affirm-force" => affirm::affirm_force(bus, param).await,
        "affirm-raw" => affirm::affirm_raw(bus, param).await,
        "affirmations" => affirm::affirmations(bus, param).await,
        "confirm" => confirm::confirm(bus, param).await,
        "ecdsa" => ecdsa::ecdsa(bus, param).await,
        "info-d2e" => info::d2e(bus, param).await,
        "keys" => keys::keys(bus, param).await,
        "sign-mmr-root" => mmr::sign_mmr_root(bus, param).await,
        "guard" => guard::guard(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}
