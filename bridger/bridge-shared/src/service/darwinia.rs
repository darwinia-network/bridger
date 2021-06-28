use std::marker::PhantomData;

use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_component::Component;
use bridge_config::config::component::BeeConfig;
use bridge_config::Config;
use bridge_standard::bridge::chain::LikeDarwiniaChain;
use bridge_standard::bridge::component::BridgeComponent;
use bridge_standard::bridge::config::BridgeConfig;
use bridge_standard::bridge::sand::BridgeSand;

use crate::bus::SharedBus;
use crate::material::MaterialDarwinia;
use crate::messages::{DarwiniaMessage, SharedMessage};
use crate::traits::{SharedKeepService, SharedMaterial};

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
        let component_bee =
            Component::bee::<MaterialDarwinia, <MaterialDarwinia as SharedMaterial>::Chain>()?;

        let _lifeline_extrinsic = Self::try_task(
            &format!("{}-extrinsic", MaterialDarwinia::NAME),
            async move {
                // let bee = component_bee.component().await?; //
                while let Some(shared_message) = rx.recv().await {
                    if let SharedMessage::Darwinia(message) = shared_message {
                        debug!(
                            target: MaterialDarwinia::NAME,
                            "recv message: {:?}", message
                        );
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
