use lifeline::prelude::*;
use lifeline::{Receiver, Sender};

use bridge_standard::bridge::task::BridgeSand;
use linked_darwinia::bus::DarwiniaLinkedBus;
use linked_darwinia::message::DarwiniaLinkedMessage;
use linked_darwinia::task::DarwiniaLinked;

use crate::message::darwinia::ToDarwiniaLinkedMessage;
use crate::task::DarwiniaEthereumTask;

lifeline_bus!(pub struct DarwiniaEthereumBus);

impl CarryFrom<DarwiniaEthereumBus> for DarwiniaLinkedBus {
    type Lifeline = anyhow::Result<lifeline::Lifeline>;

    fn carry_from(&self, from: &DarwiniaEthereumBus) -> Self::Lifeline {
        let mut rx_task = from.rx::<ToDarwiniaLinkedMessage>()?;
        let mut tx_link = self.tx::<DarwiniaLinkedMessage>()?;

        let lifeline = Self::try_task(
            &format!(
                "{}-carry-{}",
                DarwiniaEthereumTask::NAME,
                DarwiniaLinked::NAME
            ),
            async move {
                while let Some(message) = rx_task.recv().await {
                    match message {
                        ToDarwiniaLinkedMessage::SendExtrinsic => {
                            tx_link.send(DarwiniaLinkedMessage::SendExtrinsic).await?
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(lifeline)
    }
}
