#[cfg(feature = "ethlike-v1")]
pub use self::ethlike_v1::*;

#[cfg(feature = "ethlike-v1")]
mod ethlike_v1 {
    use codec::Encode;

    use crate::types::{BetterRelayAffirmation, EcdsaAddress, _S};

    /// affirmations contains block?
    pub fn affirmations_contains_block(
        affirmations: &[BetterRelayAffirmation],
        block: u64,
    ) -> bool {
        for affirmation in affirmations {
            let blocks: &Vec<u64> = &affirmation
                .relay_header_parcels
                .iter()
                .map(|bp| bp.header.number)
                .collect();
            if blocks.contains(&block) {
                return true;
            }
        }

        // TODO: Checking the equality of the affirmations

        // TODO: If there is an affirmation with larger block number, then agree and join in the game.

        // TODO: How to play and join the game
        false
    }

    /// encode mmr root message
    pub fn encode_mmr_root_message(
        spec_name: impl AsRef<str>,
        block_number: u32,
        mmr_root: subxt::sp_core::H256,
    ) -> Vec<u8> {
        let spec_name = spec_name.as_ref();
        let op_code: [u8; 4] = [71, 159, 189, 249];
        tracing::debug!(
            target: "client-pangolin",
            "Infos to construct mmr_root message: {}, {}, {}, {:?}",
            spec_name,
            array_bytes::bytes2hex("", &op_code),
            block_number,
            mmr_root
        );
        // scale encode & sign
        let message = _S {
            _1: spec_name,
            _2: op_code,
            _3: block_number,
            _4: mmr_root,
        };
        let encoded: &[u8] = &message.encode();
        encoded.to_vec()
    }

    pub fn encode_authorities_message(
        spec_name: impl AsRef<str>,
        term: u32,
        next_authorities: Vec<EcdsaAddress>,
    ) -> Vec<u8> {
        let spec_name = spec_name.as_ref();
        let op_code: [u8; 4] = [180, 188, 244, 151];
        tracing::debug!(
            target: "client-pangolin",
            "Infos to construct eth authorities message: {}, {}, {}, {:?}",
            spec_name,
            array_bytes::bytes2hex("", &op_code),
            term,
            next_authorities
                .iter()
                .map(|a| array_bytes::bytes2hex("", &a))
                .collect::<Vec<_>>()
                .join(", ")
        );
        // scale encode & sign
        let message = _S {
            _1: spec_name,
            _2: op_code,
            _3: term,
            _4: next_authorities,
        };
        let encoded: &[u8] = &message.encode();
        encoded.to_vec()
    }
}
