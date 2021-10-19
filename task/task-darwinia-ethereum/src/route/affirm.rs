use lifeline::dyn_bus::DynBus;
use lifeline::Bus;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use bridge_traits::error::StandardError;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_ethereum::errors::BizError;
use component_shadow::ShadowComponent;
use component_state::state::BridgeState;
use support_ethereum::block::EthereumHeader;
use support_ethereum::parcel::EthereumRelayHeaderParcel;

use crate::bus::DarwiniaEthereumBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::affirm::handler::AffirmHandler;
use crate::task::DarwiniaEthereumTask;

pub async fn affirm(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let block = param["block"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `block` parameter is required".to_string()))?;

    // State
    let state = bus.storage().clone_resource::<BridgeState>()?;
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

    let mut handler = AffirmHandler::new(microkv, sender_to_extrinsics).await;

    handler.do_affirm(block).await?;

    Ok(TaskTerminal::new("success"))
}

pub async fn affirm_force(
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

    let parcel = shadow.parcel(block as usize + 1).await?;
    let block_number = parcel.header.number;
    if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
        return Err(BizError::ParcelFromShadowIsEmpty(block).into());
    }
    let ex_hash = ethereum_to_darwinia
        .affirm(&from_ethereum_account, parcel)
        .await?;
    Ok(TaskTerminal::new(format!(
        "Affirmed ethereum block {} in extrinsic {:?}",
        block_number, ex_hash
    )))
}

pub async fn affirm_raw(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let json = param["json"]
        .as_str()
        .ok_or_else(|| StandardError::Api("The `json` parameter is required".to_string()))?;

    let state = bus.storage().clone_resource::<BridgeState>()?;
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

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

    // build from json string
    let parcel: EthereumRelayHeaderParcel = serde_json::from_str(json)
        .map_err(|e| StandardError::Api(format!("Failed to deserde json: {:?}", e)))?;

    // affirm
    let hash = ethereum_to_darwinia
        .affirm(&from_ethereum_account, parcel)
        .await?;
    Ok(TaskTerminal::new(format!("Extrinsic hash: {:?}", hash)))
}

pub async fn affirmations(
    _bus: &DarwiniaEthereumBus,
    _param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    // Darwinia client
    let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let darwinia = component_darwinia.component().await?;
    let ethereum_to_darwinia = Ethereum2Darwinia::new(darwinia);

    let mut output = vec![];
    for (game_id, game) in ethereum_to_darwinia.affirmations().await?.iter() {
        output.push(format!("--- GAME {} ---", game_id));
        for (round_id, affirmations) in game.iter() {
            output.push(format!("ROUND {}", round_id));
            for affirmation in affirmations {
                output.push(format!("affirmation: {:?}\n", affirmation));
            }
        }
    }

    Ok(TaskTerminal::new(output.join("\n")))
}
