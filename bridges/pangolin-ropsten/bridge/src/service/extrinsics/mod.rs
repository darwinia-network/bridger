use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use component_state::state::BridgeState;
use support_lifeline::service::BridgeService;

use crate::bridge::PangolinRopstenBus;
use crate::bridge::PangolinRopstenTask;
use crate::bridge::ToExtrinsicsMessage;
use crate::service::extrinsics::handler::ExtrinsicsHandler;

mod handler;

#[derive(Debug)]
pub struct ExtrinsicsService {
    _greet: Lifeline,
    _consume: Lifeline,
}

impl BridgeService for ExtrinsicsService {}

impl Service for ExtrinsicsService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    #[allow(irrefutable_let_patterns)]
    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToExtrinsicsMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let greet_state = state.clone();
        let _greet = Self::try_task(
            &format!("{}-service-extrinsics-collect", PangolinRopstenTask::name()),
            async move {
                let handler = ExtrinsicsHandler::new(greet_state.clone()).await;
                while let Some(recv) = rx.recv().await {
                    if let ToExtrinsicsMessage::Extrinsic(ex) = recv {
                        if handler.collect_message(&ex).is_err() {
                            tracing::info!(
                                target: "pangolin-ropsten",
                                "Failed to save extrinsic {:?}",
                                &ex
                            );
                        }
                    }
                }

                Ok(())
            },
        );

        let _consume = Self::try_task(
            &format!("{}-service-extrinsics-consume", PangolinRopstenTask::name()),
            async move {
                let mut handler = ExtrinsicsHandler::new(state.clone()).await;
                while handler.consume_message().await.is_err() {
                    tracing::error!(
                        target: "pangolin-ropsten",
                        "Failed to consume extrinsics in database"
                    );
                    handler = ExtrinsicsHandler::new(state.clone()).await;
                }
                Ok(())
            },
        );
        Ok(Self { _greet, _consume })
    }
}
