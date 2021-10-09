use component_pangolin_subxt::darwinia::client::Darwinia;
use support_ethereum::transaction::EthereumTransaction;

pub async fn is_verified(client: &Darwinia, tx: &EthereumTransaction) -> anyhow::Result<bool> {
    let verified = client.verified(tx.block_hash, tx.index).await?
        || client.verified_issuing(tx.block_hash, tx.index).await?;
    Ok(verified)
}
