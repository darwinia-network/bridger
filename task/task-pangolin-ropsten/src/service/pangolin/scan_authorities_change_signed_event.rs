use bridge_traits::bridge::component::BridgeComponent;
use component_ethereum::ethereum::EthereumComponent;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_subquery::SubqueryComponent;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;
use std::convert::TryInto;

use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;
use crate::task::PangolinRopstenTask;

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
    pub async fn start(&mut self) {}
    async fn run(&mut self) -> anyhow::Result<()> {
        // subquery
        let component_subquery = SubqueryComponent::restore::<PangolinRopstenTask>()?;
        let subquery = component_subquery.component().await?;

        // darwinia
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let darwinia = component_pangolin_subxt.component().await?;

        // ethereum

        let component_ethereum = EthereumComponent::restore::<PangolinRopstenTask>()?;
        let ethereum = component_ethereum.component().await?;

        let spec_name = darwinia.runtime_version().await?;
        let current_term = darwinia2ethereum.get_current_authority_term().await?;

        loop {
            let from = self.tracker.current().await?;
            let limit = 10usize;
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Track pangolin AuthoritiesChangeSignedEvent block: {} and limit: {}",
                from,
                limit
            );
            let events = subquery
                .query_authorities_change_signed_event(from as u64, limit as u32)
                .await?;

            for event in events {
                if event.term != current_term {
                    log::trace!(
                        target: PangolinRopstenTask::NAME,
                        "[pangolin] Queried AuthoritiesChangeSignedEvent but not in current term. the event term is {} and current term is {}. skip this.",
                        event.term,
                        current_term
                    );
                    continue;
                }

                let mut new_authorities = Vec::with_capacity(event.new_authorities.len());
                for item in event.new_authorities {
                    let message = item.as_slice().try_into()?;
                    new_authorities.push(message);
                }
                let message = Darwinia2Ethereum::construct_authorities_message(
                    spec_name.clone(),
                    event.term,
                    new_authorities,
                );

                let raw_signatures = &event.signatures.nodes;
                let mut signatures = Vec::with_capacity(raw_signatures.len());
                for signature in raw_signatures {
                    let ecdsa_signature =
                        signature.relay_authority_signature.as_slice().try_into()?;
                    signatures.push(ecdsa_signature);
                }

                let tx_hash = ethereum.submit_authorities_set(message, signatures).await?;

                log::info!(
                    target: PangolinRopstenTask::NAME,
                    "[pangolin] Submit authorities to ethereum at block {} with tx: {}",
                    event.at_block_number,
                    tx_hash
                );
            }
        }
    }
}
