use lifeline::prelude::*;
use lifeline::{Receiver, Sender};

use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use linked_template::bus::TemplateLinkedBus;
use linked_template::message::TemplateLinkedMessage;
use linked_template::task::TemplateLinked;

use crate::message::ToTemplateLinkedMessage;
use crate::task::TemplateTask;

lifeline_bus!(pub struct TemplateTaskBus);

impl Resource<TemplateTaskBus> for BridgeState {}

impl CarryFrom<TemplateTaskBus> for TemplateLinkedBus {
    type Lifeline = anyhow::Result<lifeline::Lifeline>;

    fn carry_from(&self, from: &TemplateTaskBus) -> Self::Lifeline {
        let mut rx_task = from.rx::<ToTemplateLinkedMessage>()?;
        let mut tx_link = self.tx::<TemplateLinkedMessage>()?;

        let lifeline = Self::try_task(
            &format!("{}-carry-{}", TemplateTask::NAME, TemplateLinked::NAME),
            async move {
                while let Some(message) = rx_task.recv().await {
                    match message {
                        ToTemplateLinkedMessage::SomeEvent => {
                            tx_link.send(TemplateLinkedMessage::SomeEvent).await?
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(lifeline)
    }
}
