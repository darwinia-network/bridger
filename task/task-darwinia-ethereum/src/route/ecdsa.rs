use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use bridge_traits::error::StandardError;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::to_ethereum::Darwinia2Ethereum;
use component_ethereum::config::Web3Config;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

pub async fn ecdsa(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let message = param["message"]
        .as_str()
        .ok_or_else(|| StandardError::Api("The `message` parameter is required".to_string()))?;

    let state = bus.storage().clone_resource::<BridgeState>()?;
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    // Web3
    let config_web3: Web3Config = Config::restore(DarwiniaEthereumTask::NAME)?;

    // Darwinia client
    let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let darwinia = component_darwinia.component().await?;
    let darwinia_to_ethereum = Darwinia2Ethereum::new(darwinia);

    // Account
    let darwinia_account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key_decrypt(
            state.get_task_config_password_unwrap_or_default(DarwiniaEthereumTask::NAME)?,
        )?,
        config_darwinia.relayer_real_account,
    );

    let to_ethereum_account = component_darwinia_subxt::to_ethereum::Account::new(
        darwinia_account,
        config_darwinia.ecdsa_authority_private_key,
        config_web3.endpoint,
    );

    let message = array_bytes::hex2bytes(&message[2..])
        .map_err(|_| StandardError::Hex2Bytes("message[2..]".into()))?;
    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&message);
    darwinia_to_ethereum
        .ecdsa_sign_and_submit_signed_authorities(&to_ethereum_account, buffer)
        .await?;

    Ok(TaskTerminal::new("success"))
}
