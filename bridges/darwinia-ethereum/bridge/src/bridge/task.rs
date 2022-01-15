use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use component_state::state::{BridgeState, StateOptions};
use support_common::config::{Config, Names};
use support_lifeline::task::TaskStack;

use crate::bridge::DarwiniaEthereumBus;
use crate::bridge::DarwiniaEthereumConfig;
use crate::bridge::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::affirm::AffirmService;
use crate::service::check::CheckService;
use crate::service::darwinia::DarwiniaService;
use crate::service::extrinsics::ExtrinsicsService;
use crate::service::guard::GuardService;
use crate::service::redeem::RedeemService;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    stack: TaskStack<DarwiniaEthereumBus>,
}

impl DarwiniaEthereumTask {
    pub fn name() -> &'static str {
        "darwinia-ethereum"
    }
}

impl DarwiniaEthereumTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let state = BridgeState::new(StateOptions {
            db_name: Self::name().to_string(),
        })?;
        // check config
        let _bridge_config: DarwiniaEthereumConfig =
            Config::restore(Names::BridgeDarwiniaEthereum)?;
        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::name());

        crate::migrate::migrate(&microkv, 2)?;

        let bus = DarwiniaEthereumBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<AffirmService>()?;
        stack.spawn_service::<CheckService>()?;
        stack.spawn_service::<RedeemService>()?;
        stack.spawn_service::<GuardService>()?;
        stack.spawn_service::<DarwiniaService>()?;
        stack.spawn_service::<ExtrinsicsService>()?;

        let mut tx_scan = stack.bus().tx::<DarwiniaEthereumMessage>()?;
        tx_scan
            .send(DarwiniaEthereumMessage::Scan(EthereumScanMessage::Start))
            .await?;

        Ok(Self { stack })
    }
}

impl DarwiniaEthereumTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<DarwiniaEthereumBus> {
        &self.stack
    }
}
