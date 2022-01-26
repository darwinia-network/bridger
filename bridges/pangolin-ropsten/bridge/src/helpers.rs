use substrate_subxt::sp_core::H256;

use client_pangolin::darwinia::client::Darwinia;
use client_pangolin::types::primitive_types::H256;
use component_thegraph_liketh::types::TransactionEntity;
use support_common::error::BridgerError;

pub async fn is_verified(client: &Darwinia, tx: &TransactionEntity) -> color_eyre::Result<bool> {
    let block_hash = hex_to_h256(&tx.block_hash)?;
    let tx_index = tx.tx_index;
    let verified = client.verified(block_hash, tx_index).await?
        || client.verified_issuing(block_hash, tx_index).await?;
    Ok(verified)
}

pub fn hex_to_h256(hash: impl AsRef<str>) -> color_eyre::Result<H256> {
    let bytes = array_bytes::hex2bytes(hash.as_ref()).map_err(|_e| {
        BridgerError::Hex(format!(
            "Failed to convert hex({}) to bytes.",
            hash.as_ref()
        ))
    })?;
    Ok(H256::from_slice(&bytes))
}
