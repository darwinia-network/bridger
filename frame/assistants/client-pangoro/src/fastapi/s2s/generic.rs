use sp_finality_grandpa::AuthorityList;
use subxt::rpc::{ClientT, Subscription, SubscriptionClientT};
use subxt::{sp_core, sp_runtime};

use crate::client::PangoroClient;
use crate::error::{ClientError, ClientResult};
use crate::types::runtime_types::bp_header_chain::justification::GrandpaJustification;
use crate::types::runtime_types::bp_header_chain::InitializationData;

type Header = crate::types::runtime_types::sp_runtime::generic::header::Header<
    u32,
    sp_runtime::traits::BlakeTwo256,
>;

impl PangoroClient {
    pub async fn subscribe_justification(&self) -> ClientResult<Subscription<sp_core::Bytes>> {
        Ok(self
            .subxt()
            .rpc()
            .client
            .subscribe(
                "grandpa_subscribeJustifications",
                None,
                "grandpa_unsubscribeJustifications",
            )
            .await?)
    }

    pub async fn prepare_initialization_data(&self) -> ClientResult<InitializationData<Header>> {
        let mut subscription = self.subscribe_justification().await?;
        let justification = subscription
            .next()
            .await
            .ok_or_else(|| ClientError::Custom("The subscribe is closed".to_string()))??;
        let justification: GrandpaJustification<Header> =
            codec::Decode::decode(&mut &justification.0[..])
                .map_err(|err| ClientError::Custom(format!("Wrong justification: {:?}", err)))?;

        let (initial_header_hash, initial_header_number) = (
            justification.commit.target_hash,
            justification.commit.target_number,
        );
        let initial_header = self
            .subxt()
            .rpc()
            .header(Some(initial_header_hash))
            .await?
            .ok_or_else(|| {
                ClientError::Custom(format!(
                    "Can not get initial header by hash: {:?}",
                    initial_header_hash
                ))
            })?;
        tracing::trace!(target: "client-pangoro", "Selected initial header [{}]: {}",
            initial_header_number,
            initial_header_hash,
        );
        let initial_authorities_set = self.grandpa_authorities(initial_header_hash).await?;
        println!("-----> {:?}", initial_authorities_set);
        Err(ClientError::Custom("NONE".to_string()))
    }
}

impl PangoroClient {
    async fn grandpa_authorities(&self, at: sp_core::H256) -> ClientResult<AuthorityList> {
        let params = subxt::rpc::rpc_params![
            "GrandpaApi_grandpa_authorities",
            sp_core::Bytes(Vec::new()),
            at
        ];
        let hex: String = self
            .subxt()
            .rpc()
            .client
            .request("state_call", params)
            .await?;
        let raw_authorities_set = array_bytes::hex2bytes(hex)?;
        let authorities = codec::Decode::decode(&mut &raw_authorities_set[..]).map_err(|err| {
            ClientError::Custom(format!(
                "[DecodeAuthorities] Can not decode authorities: {:?}",
                err
            ))
        })?;
        Ok(authorities)
    }
}
