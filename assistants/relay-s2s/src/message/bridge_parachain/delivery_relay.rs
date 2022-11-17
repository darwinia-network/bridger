use bridge_s2s_traits::client::{S2SClientRelay, S2SParaBridgeClientSolochain};
use bridge_s2s_traits::strategy::RelayStrategy;

use crate::error::RelayResult;
use crate::message::common::CommonDeliveryRunner;
use crate::special::ParachainSpecialClientApi;
use crate::types::MessageDeliveryInput;

pub struct BridgeParachainDeliveryRunner<SC, TC, Strategy>
where
    SC: S2SClientRelay,
    TC: S2SParaBridgeClientSolochain,
    Strategy: RelayStrategy,
{
    common: CommonDeliveryRunner<SC, TC, ParachainSpecialClientApi<TC>, Strategy>,
}

impl<SC, TC, Strategy> BridgeParachainDeliveryRunner<SC, TC, Strategy>
where
    SC: S2SClientRelay,
    TC: S2SParaBridgeClientSolochain,
    Strategy: RelayStrategy,
{
    pub fn new(input: MessageDeliveryInput<SC, TC, Strategy>, para_id: u32) -> Self {
        let different = ParachainSpecialClientApi {
            para_id,
            client: input.client_target.clone(),
        };
        let common = CommonDeliveryRunner::new(input, different);
        Self { common }
    }

    pub async fn start(&self) -> RelayResult<()> {
        self.common.start().await
    }
}
