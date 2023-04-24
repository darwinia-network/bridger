use crate::client::PangolinClient;
use crate::config::PangolinSubxtConfig;
use crate::types::runtime_types;
use bridge_e2e_traits::{
    client::EcdsaClient,
    error::{E2EClientError, E2EClientResult},
};

#[async_trait::async_trait]
impl EcdsaClient for PangolinClient {
    type SubxtConfig = PangolinSubxtConfig;

    async fn reconnect(&mut self) -> E2EClientResult<()> {
        self.reconnect_client()
            .await
            .map_err(|e| E2EClientError::Custom(format!("{:?}", e)))?;
        Ok(())
    }

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
        tracing::trace!(target: "client-pangolin", "[is_ecdsa_authority] Block hash: {:?}", hash);
        let authorities = crate::subxt_runtime::api::storage()
            .ecdsa_authority()
            .authorities();
        let authorities = self.subxt().storage().fetch(&authorities, hash).await?;
        tracing::trace!(target: "client-pangolin", "[is_ecdsa_authority] Authorities fetched");
        match authorities {
            None => {
                Ok(false)
            },
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
    ) -> E2EClientResult<<PangolinSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            E2EClientError::Custom(format!(
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
            E2EClientError::Custom(format!("send transaction failed pangolin: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }

    async fn submit_new_message_root_signature(
        &self,
        signatures: Vec<u8>,
    ) -> E2EClientResult<<PangolinSubxtConfig as subxt::Config>::Hash> {
        let fixed_signatures: [u8; 65] = signatures.try_into().map_err(|e: Vec<u8>| {
            E2EClientError::Custom(format!(
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
        let hash = track.extrinsic_hash();
        tracing::trace!(target: "client-pangolin", "[submit_new_message_root_signature] tx hash: {:?}", hash);
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            E2EClientError::Custom(format!("send transaction failed pangolin: {:?}", e))
        })?;
        Ok(events.extrinsic_hash())
    }
}
