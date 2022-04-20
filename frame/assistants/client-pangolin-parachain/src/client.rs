use pangolin_subxt::api::RuntimeApi;
use subxt::extrinsic::SubstrateExtrinsicParams;
use subxt::sp_runtime::traits::Header;
use subxt::Client;

use crate::config::PangolinParachainSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::ethereum::EthereumApi;
use crate::types::DarwiniaAccount;

/// Pangolin client
#[derive(Clone)]
pub struct PangolinParachainClient {
    /// Runtime api
    client: Client<PangolinParachainSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangolinParachainClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinParachainSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl PangolinParachainClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangolinParachainClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<PangolinParachainSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(
        &self,
    ) -> RuntimeApi<
        PangolinParachainSubxtConfig,
        SubstrateExtrinsicParams<PangolinParachainSubxtConfig>,
    > {
        self.client.clone().to_runtime_api()
    }

    /// Ethereum api
    pub fn ethereum(&self) -> EthereumApi {
        EthereumApi::new(self)
    }
}

impl PangolinParachainClient {
    /// get mmr root
    pub async fn get_mmr_root(&self, leaf_index: u32) -> ClientResult<subxt::sp_core::H256> {
        let block_number = leaf_index + 1;
        let header = self.header_by_number(block_number).await?;

        let mmr_root = if let Some(header) = header {
            // get digest_item from header
            let log = header
                .digest()
                .logs()
                .iter()
                .find(|&x| x.as_other().is_some());
            if let Some(digest_item) = log {
                // get mmr_root from log
                let parent_mmr_root = digest_item.as_other().unwrap().to_vec();
                let parent_mmr_root = &parent_mmr_root[4..];
                if parent_mmr_root.len() != 32 {
                    return Err(ClientError::WrongMmrRootInDarwiniaHeader(
                        array_bytes::bytes2hex("", &parent_mmr_root),
                        block_number,
                    ));
                }
                let mut mmr_root: [u8; 32] = [0; 32];
                mmr_root.copy_from_slice(parent_mmr_root);
                subxt::sp_core::H256(mmr_root)
            } else {
                return Err(ClientError::NoMmrRootInDarwiniaHeader(block_number));
            }
        } else {
            return Err(ClientError::FailedToFetchDarwiniaHeader(block_number));
        };
        Ok(mmr_root)
    }

    /// Query spec name
    pub async fn spec_name(&self) -> ClientResult<String> {
        let runtime_version = self.subxt().rpc().runtime_version(None).await?;
        let spec_name = runtime_version
            .other
            .get("specName")
            .ok_or_else(|| ClientError::Other("Failed to query spec name".to_string()))?
            .as_str()
            .ok_or_else(|| {
                ClientError::Other("The spec name not found in runtime version".to_string())
            })?;
        Ok(spec_name.to_string())
    }

    /// is_tech_comm_member
    pub async fn is_tech_comm_member(
        &self,
        block_number: Option<u32>,
        account: Option<DarwiniaAccount>,
    ) -> ClientResult<bool> {
        let block_hash = self
            .subxt()
            .rpc()
            .block_hash(block_number.map(|block| block.into()))
            .await?;
        let members = self
            .runtime()
            .storage()
            .technical_committee()
            .members(block_hash)
            .await?;
        let account = account.unwrap_or_else(|| self.account.clone());
        Ok(members.contains(account.real_account()))
    }

    /// query header by block number
    pub async fn header_by_number(
        &self,
        number: u32,
    ) -> ClientResult<Option<<PangolinParachainSubxtConfig as subxt::Config>::Header>> {
        match self.subxt().rpc().block_hash(Some(number.into())).await? {
            Some(hash) => Ok(self.subxt().rpc().header(Some(hash)).await?),
            None => Ok(None),
        }
    }

    // /// get mmr root of darwinia
    // pub async fn header_mmr_gen_proof(
    //     &self,
    //     block_number_of_member_leaf: u64,
    //     block_number_of_last_leaf: u64,
    //     hash: H256,
    // ) -> ClientResult<()> {
    //     let params = &[
    //         serde_json::to_value(block_number_of_member_leaf)?,
    //         serde_json::to_value(block_number_of_last_leaf)?,
    //     ];
    //     let v: HeaderMMRRpc = self
    //         .client
    //         .rpc()
    //         .client
    //         .request("headerMMR_genProof", params)
    //         .await?;
    //     Ok(())
    // }
}
