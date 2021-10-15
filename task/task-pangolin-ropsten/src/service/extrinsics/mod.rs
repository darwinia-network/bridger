use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;

use crate::bus::PangolinRopstenBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::extrinsics::handler::ExtrinsicsHandler;
use crate::task::PangolinRopstenTask;

mod handler;

#[derive(Debug)]
pub struct ExtrinsicsService {
    _greet: Lifeline,
}

impl BridgeService for ExtrinsicsService {}

impl Service for ExtrinsicsService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    #[allow(irrefutable_let_patterns)]
    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToExtrinsicsMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-extrinsics", PangolinRopstenTask::NAME),
            async move {
                let mut handler = ExtrinsicsHandler::new(state.clone()).await;

                while let Some(recv) = rx.recv().await {
                    if let ToExtrinsicsMessage::Extrinsic(ex) = recv {
                        while let Err(err) = handler.send_extrinsic(ex.clone()).await {
                            log::error!(
                                target: PangolinRopstenTask::NAME,
                                "extrinsics err: {:#?}",
                                err
                            );

                            // TODO: Consider the errors more carefully

                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            handler = ExtrinsicsHandler::new(state.clone()).await;
                        }
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
