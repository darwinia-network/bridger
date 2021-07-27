use std::sync::Arc;

use lifeline::dyn_bus::DynBus;
use lifeline::Bus;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::ShadowComponent;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::guard::GuardService;
use crate::task::DarwiniaEthereumTask;

pub async fn guard(
    bus: &DarwiniaEthereumBus,
    _param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
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

    let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
    GuardService::guard(
        ethereum_to_darwinia,
        from_ethereum_account,
        Arc::new(shadow),
        sender_to_extrinsics,
    )
    .await?;

    Ok(TaskTerminal::new("success"))
}
