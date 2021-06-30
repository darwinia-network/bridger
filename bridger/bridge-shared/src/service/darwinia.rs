use lifeline::{Bus, Lifeline, Receiver, Task};

use bridge_component::Component;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;
use chain_darwinia::DarwiniaChain;

use crate::bus::SharedBus;
use crate::messages::SharedMessage;
use crate::shared::SharedTask;

#[derive(Debug)]
pub struct DarwiniaSharedService {
    _lifeline_extrinsic: Lifeline,
}

impl BridgeService for DarwiniaSharedService {}

impl lifeline::Service for DarwiniaSharedService {
    type Bus = SharedBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<SharedMessage>()?;
        let _component_bee = Component::bee::<SharedTask, DarwiniaChain>()?;

        let _lifeline_extrinsic =
            Self::try_task(&format!("{}-extrinsic", SharedTask::NAME), async move {
                // let bee = component_bee.component().await?; //
                while let Some(shared_message) = rx.recv().await {
                    match shared_message {
                        SharedMessage::Darwinia(message) => {
                            debug!(target: SharedTask::NAME, "recv message: {:?}", message);
                        }
                    }
                }
                Ok(())
            });
        Ok(Self {
            _lifeline_extrinsic,
        })
    }
}
