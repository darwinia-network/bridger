use lifeline::Message;
use postage::broadcast;

use crate::material::darwinia::MaterialDarwinia;
use crate::traits::SharedMaterial;

#[derive(Debug, Clone)]
pub enum SharedMessage {
    Darwinia(DarwiniaMessage),
}

impl Message<<MaterialDarwinia as SharedMaterial>::Bus> for SharedMessage {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone)]
pub enum DarwiniaMessage {
    SendExtrinsic,
}
