use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask};
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::TaskConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::darwinia::DarwiniaService;
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::extrinsics::ExtrinsicsService;
use crate::service::guard::GuardService;
use crate::service::redeem::RedeemService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;
use crate::task::DarwiniaEthereumTask;

#[derive(Debug)]
pub struct StarterService {
    _greet: Lifeline,
}

impl BridgeService for StarterService {}

impl Service for StarterService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // task config
        let config_task: TaskConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
        // State
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let mut tx_scan = bus.tx::<DarwiniaEthereumMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-starter", DarwiniaEthereumTask::NAME),
            async move {
                if !config_task.is_enable_crypto() {
                    return start_services(&mut tx_scan).await;
                }

                loop {
                    let password = state.get_task_config_password(DarwiniaEthereumTask::NAME)?;
                    if password.is_some() {
                        break;
                    }
                    let timeout_secs = 3;
                    log::warn!(
                        target: DarwiniaEthereumTask::NAME,
                        "The password is required if is enabled crypto, will check after {} seconds.",
                        timeout_secs
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(timeout_secs)).await;
                }
                start_services(&mut tx_scan).await
            },
        );
        Ok(Self { _greet })
    }
}

async fn start_services<S>(tx_scan: &mut S) -> anyhow::Result<()>
where
    S: lifeline::Sender<DarwiniaEthereumMessage>,
{
    // wait task has be keep
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let task: &mut DarwiniaEthereumTask =
        support_keep::task::running_task_downcast_mut(DarwiniaEthereumTask::NAME)?;
    let stack = task.stack();

    stack.spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>()?;
    stack.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService>()?;
    stack.spawn_service::<RedeemService>()?;
    stack.spawn_service::<GuardService>()?;
    stack.spawn_service::<DarwiniaService>()?;
    stack.spawn_service::<ExtrinsicsService>()?;

    tx_scan
        .send(DarwiniaEthereumMessage::Scan(EthereumScanMessage::Start))
        .await?;
    Ok(())
}
