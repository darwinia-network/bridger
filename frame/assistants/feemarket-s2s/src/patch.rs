use frame_support::StorageHasher;

// todo: may don't need there
/// Returns the storage prefix for a specific pallet name and storage name.
///
/// The storage prefix is `concat(twox_128(pallet_name), twox_128(storage_name))`.
pub fn storage_prefix(pallet_name: &[u8], storage_name: &[u8]) -> [u8; 32] {
    let pallet_hash = frame_support::Twox128::hash(pallet_name);
    let storage_hash = frame_support::Twox128::hash(storage_name);

    let mut final_key = [0u8; 32];
    final_key[..16].copy_from_slice(&pallet_hash);
    final_key[16..].copy_from_slice(&storage_hash);

    final_key
}
