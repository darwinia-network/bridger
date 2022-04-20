use lifeline::{Lifeline, Service, Task};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{PangolinRococoBus, PangolinRococoTask};

#[derive(Debug)]
pub struct PangolinToParachainHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToParachainHeaderRelayService {}

impl Service for PangolinToParachainHeaderRelayService {
    type Bus = PangolinRococoBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!(
                "{}-pangolin-parachain-header-relay",
                PangolinRococoTask::name()
            ),
            async move {
                if let Err(e) = start() {
                    tracing::error!(
                        target: "pangolin-rococo",
                        "{:?}",
                        e,
                    );
                    return Err(BridgerError::Custom(
                        "Failed to start header relay service".to_string(),
                    )
                    .into());
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

fn start() -> color_eyre::Result<()> {
    todo!()
}
