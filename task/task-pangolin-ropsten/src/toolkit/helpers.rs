use component_pangolin_subxt::darwinia::client::Darwinia;
use component_thegraph_liketh::types::TransactionEntity;
use substrate_subxt::sp_core::H256;

pub async fn is_verified(client: &Darwinia, tx: &TransactionEntity) -> anyhow::Result<bool> {
    let block_hash = hex_to_h256(&tx.block_hash)?;
    let tx_index = tx.tx_index;
    let verified = client.verified(block_hash, tx_index).await?
        || client.verified_issuing(block_hash, tx_index).await?;
    Ok(verified)
}

pub fn hex_to_h256(hash: impl AsRef<str>) -> anyhow::Result<H256> {
    Ok(H256::from_slice(&array_bytes::hex2bytes(hash)?))
}
