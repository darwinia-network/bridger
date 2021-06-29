use std::fmt::{Debug, Formatter};

use lifeline::Sender;

use crate::messages::{DarwiniaMessage, SharedMessage};

#[derive(Clone)]
pub struct SharedChannel {
    sender: postage::broadcast::Sender<SharedMessage>,
}

impl Debug for SharedChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("SharedChannel { sender: <...> }")?;
        Ok(())
    }
}

impl SharedChannel {
    pub fn new(sender: postage::broadcast::Sender<SharedMessage>) -> Self {
        Self { sender }
    }
}

impl SharedChannel {
    pub async fn send(&mut self, message: SharedMessage) -> anyhow::Result<()> {
        self.sender.send(message).await?;
        Ok(())
    }
    pub async fn send_darwinia(&mut self, message: DarwiniaMessage) -> anyhow::Result<()> {
        self.send(SharedMessage::Darwinia(message)).await
    }
}
