use substrate_subxt::ClientBuilder;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::darwinia::runtime::DarwiniaRuntime;
use component_darwinia_subxt::frame::sudo::KeyStoreExt;
use component_darwinia_subxt::frame::technical_committee::MembersStoreExt;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

pub async fn keys(
    _bus: &DarwiniaEthereumBus,
    _param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(config_darwinia.endpoint)
        .build()
        .await?;
    let sudo = client.key(None).await?;
    // let sudo_ss58 = sudo.to_string();
    let technical_committee_members = client.members(None).await?;

    let output = vec![
        format!("sudo key: {:?}", sudo),
        format!(
            "technical committee members: {:?}",
            technical_committee_members
        ),
    ];

    Ok(TaskTerminal::new(output.join("\n")))
}
