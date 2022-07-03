use kiln_client::client::KilnClient;
use pangoro_client::client::PangoroClient;
use service::header_relay::kiln_to_pangoro::HeaderRelay;
use structopt::StructOpt;

use crate::command::types::Opts;

mod bridge;
mod cli;
mod command;
mod service;
mod pangoro_client;
mod kiln_client;

fn test_client() -> (KilnClient, PangoroClient) {
    (
        KilnClient::new("http://localhost:5052").unwrap(),
        PangoroClient::new(
            "https://pangoro-rpc.darwinia.network",
            "/Users/furoxr/Projects/bridger/frame/abstract/bridge-s2e/src/BeaconLightClient_abi.json",
            "0xedD0683d354b2d2c209Ac8c574ef88E85bdBEa70",
            "03454001267e888193ea585845b6634d8977f56040199a55ba3c8654776efed8"
        ).unwrap()
    )
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    // let opt = Opts::from_args();
    // cli::execute(opt).await?;
    let (kiln_client, pangoro_client) = test_client();
    let header_relay_service = HeaderRelay {pangoro_client, kiln_client};
    let result = header_relay_service.header_relay().await;
    Ok(())
}
