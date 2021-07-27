use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use bridge_traits::error::StandardError;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::ShadowComponent;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

pub async fn confirm(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let block = param["block"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `block` parameter is required".to_string()))?;

    let state = bus.storage().clone_resource::<BridgeState>()?;
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    // Shadow
    let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;
    let shadow = component_shadow.component().await?;

    // Darwinia client
    let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let darwinia = component_darwinia.component().await?;
    let ethereum_to_darwinia = Ethereum2Darwinia::new(darwinia);

    // Account
    let darwinia_account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key_decrypt(
            state.get_task_config_password_unwrap_or_default(DarwiniaEthereumTask::NAME)?,
        )?,
        config_darwinia.relayer_real_account,
    );
    let from_ethereum_account =
        component_darwinia_subxt::from_ethereum::Account::new(darwinia_account);

    info!("Init darwinia API succeed!");
    let parcel = shadow.parcel(block as usize).await?;
    info!("Init shadow API succeed!");
    ethereum_to_darwinia
        .set_confirmed_parcel(&from_ethereum_account, parcel)
        .await?;

    let msg = format!("Set confirmed block {} succeed!", block);
    info!("{}", msg);
    Ok(TaskTerminal::new(msg))
}
