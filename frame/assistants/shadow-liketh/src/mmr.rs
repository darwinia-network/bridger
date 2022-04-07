use ckb_merkle_mountain_range::helper;

// copy from https://github.com/darwinia-network/shadow/pull/165

type Hash = [u8; 32];

pub fn merge(lhs: &[u8], rhs: &[u8]) -> Hash {
    let mut data = vec![];
    data.append(&mut lhs.to_vec());
    data.append(&mut rhs.to_vec());
    let mut dest = [0; 32];
    dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], &data).as_bytes());
    dest
}

pub fn bag_rhs_peaks(mut peaks: Vec<Hash>) -> ckb_merkle_mountain_range::Result<Hash> {
    let last = peaks
        .pop()
        .ok_or_else(|| ckb_merkle_mountain_range::Error::GenProofForInvalidLeaves)?;
    peaks.reverse();
    Ok(peaks.iter().fold(last, |prev, &next| merge(&prev, &next)))
}

// mmr root
// 1. helper::get_peaks(leaf_pos);
// 2. query hash of peaks from db
// 3. bag_rhs_peaks

// first call gen_proof_positions to get the positions proof needed
pub fn gen_proof_positions(mut pos: u64, leaf_pos: u64) -> (Vec<u64>, Vec<u64>, u64) {
    let mut height = 0;
    let mut proof_index = Vec::new();
    while pos < leaf_pos - 1 {
        let pos_height = helper::pos_height_in_tree(pos);
        let next_height = helper::pos_height_in_tree(pos + 1);
        if next_height > pos_height {
            // left child sib
            let sib = pos - helper::sibling_offset(height);
            if sib > leaf_pos - 1 {
                break;
            }
            proof_index.push(sib);
            pos += 1;
        } else {
            // right child sib
            let sib = pos + helper::sibling_offset(height);
            if sib > leaf_pos - 1 {
                break;
            }
            proof_index.push(sib);
            pos += 2 << height;
        }
        height += 1;
    }
    let peak_positions = helper::get_peaks(leaf_pos);
    return (proof_index, peak_positions, pos);
}

// after get positions by gen_proof_positions, query the hash of the positions and then gen_proof
pub fn gen_proof(merkle_proof: Vec<Hash>, peaks: Vec<(u64, Hash)>, peak_pos: u64) -> Vec<Hash> {
    let mut proof = peaks
        .iter()
        .filter(|(pos, _)| pos < &peak_pos)
        .map(|(_, h)| *h)
        .collect::<Vec<Hash>>();
    proof.append(&mut merkle_proof.clone());
    let rhs_peaks = peaks
        .iter()
        .filter(|(pos, _)| pos > &peak_pos)
        .map(|(_, h)| *h)
        .collect::<Vec<Hash>>();
    if let Ok(rhs_peak_hash) = bag_rhs_peaks(rhs_peaks) {
        proof.push(rhs_peak_hash);
    }
    return proof;
}
