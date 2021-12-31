use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use component_state::state::BridgeState;
use support_lifeline::service::BridgeService;

use crate::bridge::DarwiniaEthereumBus;
use crate::bridge::DarwiniaEthereumTask;
use crate::bridge::ToExtrinsicsMessage;
use crate::service::extrinsics::handler::ExtrinsicsHandler;

mod handler;

#[derive(Debug)]
pub struct ExtrinsicsService {
    _greet: Lifeline,
}

impl BridgeService for ExtrinsicsService {}

impl Service for ExtrinsicsService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = color_eyre::Result<Self>;

    #[allow(irrefutable_let_patterns)]
    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToExtrinsicsMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-extrinsics", DarwiniaEthereumTask::name()),
            async move {
                let mut handler = ExtrinsicsHandler::new(state.clone()).await;

                while let Some(recv) = rx.recv().await {
                    if let ToExtrinsicsMessage::Extrinsic(ex) = recv {
                        while let Err(err) = handler.send_extrinsic(ex.clone()).await {
                            tracing::error!(
                                target: "darwinia-ethereum",
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
