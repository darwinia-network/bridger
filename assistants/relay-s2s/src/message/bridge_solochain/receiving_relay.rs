use bridge_s2s_traits::client::S2SClientRelay;

use crate::error::RelayResult;
use crate::message::common::CommonReceivingRunner;
use crate::special::SolochainSpecialClientApi;
use crate::types::MessageReceivingInput;

pub struct BridgeSolochainReceivingRunner<SC, TC>
where
    SC: S2SClientRelay,
    TC: S2SClientRelay,
{
    common: CommonReceivingRunner<SC, TC, SolochainSpecialClientApi<SC>>,
}

impl<SC, TC> BridgeSolochainReceivingRunner<SC, TC>
where
    SC: S2SClientRelay,
    TC: S2SClientRelay,
{
    pub fn new(input: MessageReceivingInput<SC, TC>) -> Self {
        let different = SolochainSpecialClientApi {
            client: input.client_source.clone(),
        };
        let common = CommonReceivingRunner::new(input, different);
        Self { common }
    }

    pub async fn start(&self) -> RelayResult<()> {
        self.common.start().await
    }
}
