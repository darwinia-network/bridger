use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::ClientError;
use crate::types::runtime_types;

impl PangoroClient {
    /// current account is ecdsa authority account
    pub async fn is_ecdsa_authority(
        &self,
        block_number: Option<u32>,
        your_address: &[u8; 20],
    ) -> ClientResult<bool> {
        let hash = self.subxt().rpc().block_hash(block_number).await?;
        let authorities = self
            .runtime()
            .storage()
            .ecdsa_authority()
            .authorities(hash)
            .await?;
        let authorities = authorities.0;
        let iam = authorities.iter().any(|&item| {
            let authority: [u8; 20] = item.0;
            authority.as_slice() == your_address
        });
        Ok(iam)
    }

    pub async fn submit_authorities_change_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> ClientResult<<PangoroSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let track = self
            .runtime()
            .tx()
            .ecdsa_authority()
            .submit_authorities_change_signature(
                runtime_types::primitive_types::H160(address),
                runtime_types::sp_core::ecdsa::Signature(fixed_signatures),
            )
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }

    pub async fn submit_new_message_root_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> ClientResult<<PangoroSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let track = self
            .runtime()
            .tx()
            .ecdsa_authority()
            .submit_new_message_root_signature(
                runtime_types::primitive_types::H160(address),
                runtime_types::sp_core::ecdsa::Signature(fixed_signatures),
            )
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }
}
