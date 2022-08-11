use crate::client::PangoroClient;

impl PangoroClient {
    /// current account is ecdsa authority account
    // todo: there used wrong account address. please use ethereum address
    pub async fn is_ecdsa_authority(&self, block_number: Option<u32>) -> ClientResult<bool> {
        let hash = self.subxt().rpc().block_hash(block_number).await?;
        let authorities = self
            .runtime()
            .storage()
            .ecdsa_authority()
            .authorities(hash)
            .await?;
        let authorities = authorities.0;
        let myself = self.account().real_account().clone();
        let myself: &[u8] = myself.as_ref();
        let iam = authorities.iter().any(|&item| {
            let authority: [u8; 20] = item.0;
            authority.as_slice() == myself
        });
        Ok(iam)
    }
}
