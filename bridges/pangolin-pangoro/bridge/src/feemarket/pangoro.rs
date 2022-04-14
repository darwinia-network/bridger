use bp_messages::{LaneId, MessageNonce};
use codec::{Decode, Encode};
use frame_support::Blake2_128Concat;
use relay_pangoro_client::PangoroChain;
use relay_substrate_client::{ChainBase, Client, TransactionSignScheme};
use relay_utils::relay_loop::Client as RelayLoopClient;
use scale_info::TypeInfo;
use sp_core::storage::StorageKey;
use sp_core::Pair;

use feemarket_s2s::api::FeemarketApi;
use feemarket_s2s::error::FeemarketResult;

#[derive(Clone)]
pub struct PangoroFeemarketApi {
    client: Client<PangoroChain>,
    lane_id: LaneId,
    signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
}

impl PangoroFeemarketApi {
    pub fn new(
        client: Client<PangoroChain>,
        lane_id: LaneId,
        signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
    ) -> Self {
        Self {
            client,
            lane_id,
            signer,
        }
    }
}

#[async_trait::async_trait]
impl FeemarketApi for PangoroFeemarketApi {
    type Chain = PangoroChain;

    async fn reconnect(&mut self) -> FeemarketResult<()> {
        Ok(self.client.reconnect().await?)
    }

    fn lane_id(&self) -> LaneId {
        self.lane_id.clone()
    }

    async fn best_finalized_header_number(
        &self,
    ) -> FeemarketResult<<Self::Chain as ChainBase>::BlockNumber> {
        Ok(self.client.best_finalized_header_number().await?)
    }

    async fn assigned_relayers(
        &self,
    ) -> FeemarketResult<
        Vec<
            dp_fee::Relayer<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::Balance,
            >,
        >,
    > {
        let storage_key = StorageKey(
            feemarket_s2s::helpers::storage_prefix(
                "FeeMarket".as_bytes(),
                "AssignedRelayers".as_bytes(),
            )
            .to_vec(),
        );
        Ok(self
            .client
            .storage_value(storage_key, None)
            .await?
            .unwrap_or_default())
    }

    async fn my_assigned_info(
        &self,
    ) -> FeemarketResult<
        Option<(
            usize,
            dp_fee::Relayer<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::Balance,
            >,
        )>,
    > {
        let signer_id = (*self.signer.public().as_array_ref()).into();
        let assigned_relayers = self.assigned_relayers().await?;
        let ret = assigned_relayers
            .iter()
            .position(|item| item.id == signer_id)
            .map(|position| {
                (
                    position,
                    assigned_relayers
                        .get(position)
                        .cloned()
                        .expect("Unreachable"),
                )
            });
        Ok(ret)
    }

    async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> FeemarketResult<
        Option<
            dp_fee::Order<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::BlockNumber,
                <Self::Chain as ChainBase>::Balance,
            >,
        >,
    > {
        let storage_key = bp_runtime::storage_map_final_key::<Blake2_128Concat>(
            "FeeMarket",
            "Orders",
            (laned_id, message_nonce).encode().as_slice(),
        );
        Ok(self.client.storage_value(storage_key.clone(), None).await?)
    }

    async fn relayers(&self) -> FeemarketResult<Vec<<Self::Chain as ChainBase>::AccountId>> {
        let storage_key = StorageKey(
            feemarket_s2s::helpers::storage_prefix("FeeMarket".as_bytes(), "Relayers".as_bytes())
                .to_vec(),
        );
        Ok(self
            .client
            .storage_value(storage_key, None)
            .await?
            .unwrap_or_default())
    }

    async fn relayer(
        &self,
        account: <Self::Chain as ChainBase>::AccountId,
    ) -> FeemarketResult<
        Option<
            dp_fee::Relayer<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::Balance,
            >,
        >,
    > {
        let storage_key = bp_runtime::storage_map_final_key::<Blake2_128Concat>(
            "FeeMarket",
            "RelayersMap",
            account.encode().as_slice(),
        );
        Ok(self.client.storage_value(storage_key.clone(), None).await?)
    }

    async fn is_relayer(&self) -> FeemarketResult<bool> {
        let signer_id = (*self.signer.public().as_array_ref()).into();
        self.relayer(signer_id).await.map(|item| item.is_some())
    }

    async fn update_relay_fee(
        &self,
        amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        crate::chains::pangoro::s2s_feemarket::update_relay_fee(
            &self.client,
            self.signer.clone(),
            amount,
        )
        .await
    }

    async fn update_locked_collateral(
        &self,
        amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        crate::chains::pangoro::s2s_feemarket::update_locked_collateral(
            &self.client,
            self.signer.clone(),
            amount,
        )
        .await
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
enum Call {
    #[codec(index = 22)]
    Feemarket(FeemarketCall),
}

/// Feemarket call
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
enum FeemarketCall {
    #[codec(index = 0)]
    enroll_and_lock_collateral(bp_pangoro::Balance, Option<bp_pangoro::Balance>),
    #[codec(index = 1)]
    update_locked_collateral(bp_pangoro::Balance),
    #[codec(index = 2)]
    update_relay_fee(bp_pangoro::Balance),
    #[codec(index = 3)]
    cancel_enrollment(),
    #[codec(index = 4)]
    set_slash_protect(bp_pangoro::Balance),
    #[codec(index = 5)]
    set_assigned_relayers_number(u32),
}
