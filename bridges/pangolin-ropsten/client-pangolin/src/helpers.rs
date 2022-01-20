use crate::types::BetterRelayAffirmation;

/// affirmations contains block?
pub fn affirmations_contains_block(affirmations: &[BetterRelayAffirmation], block: u64) -> bool {
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
