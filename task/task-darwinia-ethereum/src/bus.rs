use lifeline::prelude::*;
use lifeline::{Receiver, Sender};

use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use linked_darwinia::bus::DarwiniaLinkedBus;
use linked_darwinia::message::DarwiniaLinkedMessage;
use linked_darwinia::task::DarwiniaLinked;

use crate::message::{DarwiniaEthereumMessage, ToDarwiniaLinkedMessage};
use crate::task::DarwiniaEthereumTask;

lifeline_bus!(pub struct DarwiniaEthereumBus);

impl Resource<DarwiniaEthereumBus> for BridgeState {}

impl CarryFrom<DarwiniaEthereumBus> for DarwiniaLinkedBus {
    type Lifeline = anyhow::Result<lifeline::Lifeline>;

    fn carry_from(&self, from: &DarwiniaEthereumBus) -> Self::Lifeline {
        let mut rx_task = from.rx::<DarwiniaEthereumMessage>()?;
        let mut tx_link = self.tx::<DarwiniaLinkedMessage>()?;

        let lifeline = Self::try_task(
            &format!(
                "{}-carry-{}",
                DarwiniaEthereumTask::NAME,
                DarwiniaLinked::NAME
            ),
            async move {
                while let Some(message_darwinia_ethereum) = rx_task.recv().await {
                    if let DarwiniaEthereumMessage::ToDarwinia(message_to_darwinia_linked) =
                        message_darwinia_ethereum
                    {
                        match message_to_darwinia_linked {
                            ToDarwiniaLinkedMessage::SendExtrinsic => {
                                tx_link.send(DarwiniaLinkedMessage::SendExtrinsic).await?
                            }
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(lifeline)
    }
}
