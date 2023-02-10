use bridge_e2e_traits::{client::EcdsaClient, error::E2EClientResult};

use crate::client::DarwiniaClient;
use crate::config::DarwiniaSubxtConfig;
use crate::error::ClientError;
use crate::types::runtime_types;

#[async_trait::async_trait]
impl EcdsaClient for DarwiniaClient {
    type SubxtConfig = DarwiniaSubxtConfig;

    /// current account is ecdsa authority account
    async fn is_ecdsa_authority(
        &self,
        block_number: Option<u32>,
        your_address: &[u8; 20],
    ) -> E2EClientResult<bool> {
        let hash = self
            .subxt()
            .rpc()
            .block_hash(block_number.map(Into::into))
            .await?;
        let authorities = crate::subxt_runtime::api::storage()
            .ecdsa_authority()
            .authorities();
        let authorities = self.subxt().storage().fetch(&authorities, hash).await?;
        match authorities {
            None => Ok(false),
            Some(authorities) => {
                let authorities = authorities.0;
                let iam = authorities.iter().any(|item| {
                    let authority: &[u8; 20] = &item.0;
                    authority == your_address
                });
                Ok(iam)
            }
        }
    }

    async fn submit_authorities_change_signature(
        &self,
        signatures: Vec<u8>,
    ) -> E2EClientResult<<DarwiniaSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let tx = crate::subxt_runtime::api::tx()
            .ecdsa_authority()
            .submit_authorities_change_signature(runtime_types::sp_core::ecdsa::Signature(
                fixed_signatures,
            ));
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch(&tx, self.account().signer(), Default::default())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            ClientError::Custom(format!("send transaction failed darwinia: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }

    async fn submit_new_message_root_signature(
        &self,
        signatures: Vec<u8>,
    ) -> E2EClientResult<<DarwiniaSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            ClientError::Custom(format!(
                "Wrong signatures data: {}",
                array_bytes::bytes2hex("0x", e.as_slice())
            ))
        })?;
        let tx = crate::subxt_runtime::api::tx()
            .ecdsa_authority()
            .submit_new_message_root_signature(runtime_types::sp_core::ecdsa::Signature(
                fixed_signatures,
            ));
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch(&tx, self.account().signer(), Default::default())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            ClientError::Custom(format!("send transaction failed darwinia: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }
}
