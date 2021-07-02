use lifeline::{Lifeline, Service, Task};

use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;

use crate::task::PangolinMillauTask;

#[derive(Debug)]
pub struct RelayService {
    _greet: Lifeline,
}

impl BridgeService for RelayService {}

impl Service for RelayService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-init-bridge", PangolinMillauTask::NAME),
            async move {
                debug!(target: PangolinMillauTask::NAME, "hello relay");

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
