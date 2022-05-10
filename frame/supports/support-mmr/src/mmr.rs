use blake2_rfc::blake2b::blake2b;
pub use cmmr::helper::{get_peaks, leaf_index_to_mmr_size, leaf_index_to_pos};
use cmmr::{helper, Error, Result};

type Hash = [u8; 32];

pub fn merge(lhs: &[u8], rhs: &[u8]) -> Hash {
    let mut data = vec![];
    data.append(&mut lhs.to_vec());
    data.append(&mut rhs.to_vec());
    let mut dest = [0; 32];
    dest.copy_from_slice(blake2b(32, &[], &data).as_bytes());
    dest
}

pub fn bag_rhs_peaks(mut peaks: Vec<Hash>) -> Result<Hash> {
    let last = peaks.pop().ok_or(Error::GenProofForInvalidLeaves)?;
    peaks.reverse();
    Ok(peaks.iter().fold(last, |prev, &next| merge(&prev, &next)))
}

// mmr root
// 1. helper::get_peaks(mmr_size);
// 2. query hash of peaks from db
// 3. bag_rhs_peaks

// first call gen_proof_positions to get the positions proof needed
pub fn gen_proof_positions(
    verified_leaf_position: u64,
    mmr_size: u64,
) -> (Vec<u64>, Vec<u64>, u64) {
    let mut height = 0;
    let mut merkle_proof_positions = Vec::new();
    let mut pos = verified_leaf_position;
    while pos < mmr_size - 1 {
        let pos_height = helper::pos_height_in_tree(pos);
        let next_height = helper::pos_height_in_tree(pos + 1);
        if next_height > pos_height {
            // left child sib
            let sib = pos - helper::sibling_offset(height);
            if sib > mmr_size - 1 {
                break;
            }
            merkle_proof_positions.push(sib);
            pos += 1;
        } else {
            // right child sib
            let sib = pos + helper::sibling_offset(height);
            if sib > mmr_size - 1 {
                break;
            }
            merkle_proof_positions.push(sib);
            pos += 2 << height;
        }
        height += 1;
    }
    let peak_positions = get_peaks(mmr_size);
    // at last pos = merkle root = peak of the subtree verified_leaf_position in
    (merkle_proof_positions, peak_positions, pos)
}

// after get positions by gen_proof_positions, query the hash of the positions and then gen_proof
pub fn gen_proof(
    mut merkle_proof: Vec<Hash>,
    peaks: Vec<(u64, Hash)>,
    peak_pos_of_verified_leaf: u64,
) -> Vec<Hash> {
    let mut proof = peaks
        .iter()
        .filter(|(pos, _)| pos < &peak_pos_of_verified_leaf)
        .map(|(_, h)| *h)
        .collect::<Vec<Hash>>();
    proof.append(&mut merkle_proof);
    let rhs_peaks = peaks
        .iter()
        .filter(|(pos, _)| pos > &peak_pos_of_verified_leaf)
        .map(|(_, h)| *h)
        .collect::<Vec<Hash>>();
    if let Ok(rhs_peak_hash) = bag_rhs_peaks(rhs_peaks) {
        proof.push(rhs_peak_hash);
    }
    proof
}
