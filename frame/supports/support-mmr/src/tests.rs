use blake2_rfc::blake2b::blake2b;
use cmmr::Merge;
use cmmr::{helper, util::MemStore, Error, MMRStore, Result, MMR};

use crate::mmr::merge;
use crate::mmr::{
    bag_rhs_peaks, gen_proof, gen_proof_positions, get_peaks, leaf_index_to_mmr_size,
};

struct MergeHash;

impl Merge for MergeHash {
    type Item = [u8; 32];
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        merge(lhs, rhs)
    }
}

fn hash(data: &[u8]) -> [u8; 32] {
    let mut dest = [0; 32];
    dest.copy_from_slice(blake2b(32, &[], data).as_bytes());
    dest
}

#[test]
fn test_mmr_root() {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
    let leaf_index: u64 = 20000;

    let _positions: Vec<u64> = (0u64..leaf_index)
        .map(|i| mmr.push(hash(&i.to_le_bytes())).unwrap())
        .collect();
    mmr.commit().unwrap();

    let mut leaf = 0u64;
    while leaf < leaf_index {
        let mmrsize = leaf_index_to_mmr_size(leaf);
        let mmr = MMR::<_, MergeHash, _>::new(mmrsize, &store);
        let mmr_root_expected = mmr.get_root();
        //1. get peaks
        let peak_positions = get_peaks(mmrsize);
        //2. query from db
        let peaks = peak_positions
            .into_iter()
            .map(|pos| {
                (&store)
                    .get_elem(pos)
                    .and_then(|elem| elem.ok_or(Error::InconsistentStore))
            })
            .collect::<Result<Vec<[u8; 32]>>>();
        // bag peaks
        let mmr_root = bag_rhs_peaks(peaks.unwrap());
        assert_eq!(mmr_root_expected, mmr_root);
        leaf += 1;
    }
}

#[test]
fn test_mmr_proof() {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
    let leaf_index: u64 = 20000;

    let _positions: Vec<u64> = (0u64..leaf_index + 1)
        .map(|i| mmr.push(hash(&i.to_le_bytes())).unwrap())
        .collect();
    let mmrsize_fromindex = leaf_index_to_mmr_size(leaf_index);
    let mmrsize = mmr.mmr_size();
    assert_eq!(mmrsize_fromindex, mmrsize);
    let root = mmr.get_root().unwrap();
    mmr.commit().unwrap();

    let mut verified_number: u64 = 0;
    let mmr = MMR::<_, MergeHash, _>::new(mmrsize, &store);
    while verified_number < leaf_index {
        let verified_position = helper::leaf_index_to_pos(verified_number);
        let mmr_proof_expected = mmr.gen_proof(vec![verified_position]).unwrap();

        // 1. gen positions
        let (merkle_proof_pos, peak_positions, peak_pos_of_leaf_index) =
            gen_proof_positions(verified_position, mmrsize);
        // 2. query hash from db
        let merkle_proof = merkle_proof_pos
            .iter()
            .map(|pos| {
                (&store)
                    .get_elem(*pos)
                    .and_then(|elem| elem.ok_or(Error::InconsistentStore))
            })
            .collect::<Result<Vec<[u8; 32]>>>();
        let peaks = peak_positions
            .iter()
            .map(|pos| (*pos, (&store).get_elem(*pos).unwrap().unwrap()))
            .collect::<Vec<(u64, [u8; 32])>>();
        // 3. gen proof
        let mmr_proof = gen_proof(merkle_proof.unwrap(), peaks, peak_pos_of_leaf_index);
        assert_eq!(mmr_proof_expected.proof_items(), mmr_proof);
        assert!(mmr_proof_expected
            .verify(
                root,
                vec![(verified_position, hash(&verified_number.to_le_bytes()))]
            )
            .unwrap());
        let mmr_proof_instance = cmmr::MerkleProof::<[u8; 32], MergeHash>::new(mmrsize, mmr_proof);
        assert!(mmr_proof_instance
            .verify(
                root,
                vec![(verified_position, hash(&verified_number.to_le_bytes()))]
            )
            .unwrap());
        verified_number += 1;
    }
}
