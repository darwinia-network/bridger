use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::types::runtime_types;

impl PangoroClient {
    /// current account is ecdsa authority account
    pub async fn is_ecdsa_authority(
        &self,
        block_number: Option<u32>,
        your_address: &[u8; 20],
    ) -> ClientResult<bool> {
        let hash = self
            .subxt()
            .rpc()
            .block_hash(block_number.map(Into::into))
            .await?;
        let authorities = self
            .runtime()
            .storage()
            .ecdsa_authority()
            .authorities(hash)
            .await?;
        let authorities = authorities.0;
        let iam = authorities.iter().any(|item| {
            let authority: &[u8; 20] = &item.0;
            authority == your_address
        });
        Ok(iam)
    }

    pub async fn submit_authorities_change_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> ClientResult<<PangoroSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .ecdsa_authority()
            .submit_authorities_change_signature(
                runtime_types::primitive_types::H160(address),
                runtime_types::sp_core::ecdsa::Signature(fixed_signatures),
            )
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            ClientError::Custom(format!("send transaction failed pangoro: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }

    pub async fn submit_new_message_root_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> ClientResult<<PangoroSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .ecdsa_authority()
            .submit_new_message_root_signature(
                runtime_types::primitive_types::H160(address),
                runtime_types::sp_core::ecdsa::Signature(fixed_signatures),
            )
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            ClientError::Custom(format!("send transaction failed pangoro: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }

    /// calaculate threshold
    pub async fn calac_threshold(&self, block_number: Option<u32>) -> ClientResult<u32> {
        let hash = self
            .subxt()
            .rpc()
            .block_hash(block_number.map(Into::into))
            .await?;
        let runtime = self.runtime();

        let threshold = runtime.constants().ecdsa_authority().sign_threshold()?;

        let authorities = runtime
            .storage()
            .ecdsa_authority()
            .next_authorities(hash)
            .await?;
        let authorities = authorities.0;
        let authorities_count = authorities.len() as u32;
        Ok(threshold.0 * authorities_count)
    }
}
