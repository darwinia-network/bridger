use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;

pub struct ScanScheduleAuthoritiesChangeEvent {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    tracker: Tracker,
}

impl ScanScheduleAuthoritiesChangeEvent {
    pub fn new(
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        microkv: NamespaceMicroKV,
    ) -> Self {
        let tracker = Tracker::new(microkv, "scan.pangolin.schedule-authorities-change");
        Self {
            sender_to_extrinsics,
            tracker,
        }
    }
}

impl ScanScheduleAuthoritiesChangeEvent {
    pub async fn start(&self) {}
}
