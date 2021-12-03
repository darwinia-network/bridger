use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;

pub struct ScanAuthoritiesChangeSignedEvent {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    tracker: Tracker,
}

impl ScanAuthoritiesChangeSignedEvent {
    pub fn new(
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        microkv: NamespaceMicroKV,
    ) -> Self {
        let tracker = Tracker::new(microkv, "scan.pangolin.authorities-change-signed");
        Self {
            sender_to_extrinsics,
            tracker,
        }
    }
}

impl ScanAuthoritiesChangeSignedEvent {
    pub async fn start(&self) {}
}
