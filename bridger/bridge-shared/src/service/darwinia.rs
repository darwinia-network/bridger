use lifeline::{Bus, Lifeline, Receiver, Task};

use bridge_component::Component;
use bridge_standard::bridge::sand::BridgeSand;

use crate::material::darwinia::MaterialDarwinia;
use crate::messages::SharedMessage;
use crate::traits::{SharedChainMaterial, SharedKeepService, SharedMaterial};

#[derive(Debug)]
pub struct DarwiniaSharedService {
    _lifeline_extrinsic: Lifeline,
}

impl SharedKeepService for DarwiniaSharedService {}

impl lifeline::Service for DarwiniaSharedService {
    type Bus = <MaterialDarwinia as SharedMaterial>::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<SharedMessage>()?;
        let _component_bee =
            Component::bee::<MaterialDarwinia, <MaterialDarwinia as SharedChainMaterial>::Chain>()?;

        let _lifeline_extrinsic = Self::try_task(
            &format!("{}-extrinsic", MaterialDarwinia::NAME),
            async move {
                // let bee = component_bee.component().await?; //
                while let Some(shared_message) = rx.recv().await {
                    match shared_message {
                        SharedMessage::Darwinia(message) => {
                            debug!(
                                target: MaterialDarwinia::NAME,
                                "recv message: {:?}", message
                            );
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self {
            _lifeline_extrinsic,
        })
    }
}
