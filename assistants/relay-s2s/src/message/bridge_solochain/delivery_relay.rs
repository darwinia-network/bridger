use bridge_s2s_traits::client::S2SClientRelay;
use bridge_s2s_traits::strategy::RelayStrategy;

use crate::error::RelayResult;
use crate::message::common::CommonDeliveryRunner;
use crate::special::SolochainSpecialClientApi;
use crate::types::MessageDeliveryInput;

pub struct BridgeSolochainDeliveryRunner<SC, TC, Strategy>
where
    SC: S2SClientRelay,
    TC: S2SClientRelay,
    Strategy: RelayStrategy,
{
    common: CommonDeliveryRunner<SC, TC, SolochainSpecialClientApi<TC>, Strategy>,
}

impl<SC, TC, Strategy> BridgeSolochainDeliveryRunner<SC, TC, Strategy>
where
    SC: S2SClientRelay,
    TC: S2SClientRelay,
    Strategy: RelayStrategy,
{
    pub fn new(input: MessageDeliveryInput<SC, TC, Strategy>) -> Self {
        let different = SolochainSpecialClientApi {
            client: input.client_target.clone(),
        };
        let common = CommonDeliveryRunner::new(input, different);
        Self { common }
    }

    pub async fn start(&self) -> RelayResult<()> {
        self.common.start().await
    }
}
