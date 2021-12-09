use std::convert::TryInto;

use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::to_ethereum::Darwinia2Ethereum;

use crate::service::darwinia::types::ScanDataWrapper;
use crate::task::DarwiniaEthereumTask;

pub struct ScanAuthoritiesChangeSignedEvent<'a> {
    data: &'a mut ScanDataWrapper,
}

impl<'a> ScanAuthoritiesChangeSignedEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanAuthoritiesChangeSignedEvent<'a> {
    pub async fn handle(&mut self) -> anyhow::Result<Option<u32>> {
        let spec_name = self.data.darwinia.runtime_version().await?;
        let current_term = self
            .data
            .darwinia2ethereum
            .get_current_authority_term()
            .await?;

        let events = self
            .data
            .subquery
            .query_authorities_change_signed_event(self.data.from, self.data.limit)
            .await?;

        log::debug!(
            target: DarwiniaEthereumTask::NAME,
            "[darwinia] Track darwinia AuthoritiesChangeSignedEvent block: {} and limit: {}",
            self.data.from,
            self.data.limit
        );
        if events.is_empty() {
            log::info!(
                target: DarwiniaEthereumTask::NAME,
                "[darwinia] Not have more AuthoritiesChangeSignedEvent"
            );
            return Ok(None);
        }

        for event in &events {
            if event.term != current_term {
                log::info!(
                    target: DarwiniaEthereumTask::NAME,
                    "[darwinia] Queried AuthoritiesChangeSignedEvent but not in current term. the event term is {} and current term is {}. skip this.",
                    event.term,
                    current_term
                );
                continue;
            }
            log::trace!(
                target: DarwiniaEthereumTask::NAME,
                "[darwinia] Processing authorities change signed event in block {}",
                event.at_block_number
            );

            let mut new_authorities = Vec::with_capacity(event.new_authorities.len());
            for item in &event.new_authorities {
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
                let ecdsa_signature = signature.relay_authority_signature.as_slice().try_into()?;
                signatures.push(ecdsa_signature);
            }

            let tx_hash = self
                .data
                .ethereum
                .submit_authorities_set(message, signatures)
                .await?;

            log::info!(
                target: DarwiniaEthereumTask::NAME,
                "[darwinia] Submit authorities to ethereum at block {} with tx: {}",
                event.at_block_number,
                tx_hash
            );
        }
        let latest = events.last().unwrap();
        Ok(Some(latest.at_block_number))
    }
}
