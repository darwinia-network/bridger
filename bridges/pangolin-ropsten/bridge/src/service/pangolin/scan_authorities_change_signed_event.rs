use std::convert::TryInto;

use crate::service::pangolin::types::ScanDataWrapper;

pub struct ScanAuthoritiesChangeSignedEvent<'a> {
    data: &'a mut ScanDataWrapper,
}

impl<'a> ScanAuthoritiesChangeSignedEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanAuthoritiesChangeSignedEvent<'a> {
    pub async fn handle(&mut self) -> color_eyre::Result<Option<u32>> {
        let client = &self.data.pangolin;

        let current_term = client
            .ethereum()
            .ethereum_relay_authorities_next_term()
            .await?;

        let events = self
            .data
            .subquery
            .query_authorities_change_signed_event(self.data.from, self.data.limit)
            .await?;

        tracing::debug!(
            target: "pangolin-ropsten",
            "[pangolin] [authorities-change] Track pangolin AuthoritiesChangeSignedEvent block: {} and limit: {}",
            self.data.from,
            self.data.limit
        );
        if events.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [authorities-change] Not have more AuthoritiesChangeSignedEvent"
            );
            return Ok(None);
        }

        for event in &events {
            if event.term != current_term {
                tracing::info!(
                    target: "pangolin-ropsten",
                    "[pangolin] [authorities-change] Queried AuthoritiesChangeSignedEvent but not in current term. the event term is {} and current term is {}. skip this.",
                    event.term,
                    current_term
                );
                continue;
            }
            tracing::trace!(
                target: "pangolin-ropsten",
                "[pangolin] [authorities-change] Processing authorities change signed event in block {}",
                event.at_block_number
            );

            let mut new_authorities = Vec::with_capacity(event.new_authorities.len());
            for item in &event.new_authorities {
                let message = item.as_slice().try_into()?;
                new_authorities.push(message);
            }
            let spec_name = client.spec_name().await?;
            let message = client_pangolin::helpers::encode_authorities_message(
                spec_name,
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

            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [authorities-change] Submit authorities to ethereum at block {} with tx: {}",
                event.at_block_number,
                tx_hash
            );
        }
        let latest = events.last().unwrap();
        Ok(Some(latest.at_block_number))
    }
}
