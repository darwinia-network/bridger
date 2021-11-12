use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;
use component_pangoro_s2s::api::PangoroApi;
use component_pangoro_s2s::PangoroChain;
use relay_substrate_client::TransactionSignScheme;

#[derive(Clone)]
pub struct ApiHelper {
    pub pangolin_api: PangolinApi,
    pub pangoro_api: PangoroApi,
    pub pangolin_signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
    pub pangoro_signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
}

#[async_trait::async_trait]
pub trait UpdateFeeStrategy {
    async fn handle(&self, helper: ApiHelper) -> anyhow::Result<()>;
}
