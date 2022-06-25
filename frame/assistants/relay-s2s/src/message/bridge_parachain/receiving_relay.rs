use abstract_bridge_s2s::client::{S2SClientRelay, S2SParaBridgeClientSolochain};

use crate::error::RelayResult;
use crate::message::common::CommonReceivingRunner;
use crate::special::ParachainSpecialClientApi;
use crate::types::MessageReceivingInput;

pub struct BridgeParachainReceivingRunner<SC, TC>
where
    SC: S2SParaBridgeClientSolochain,
    TC: S2SClientRelay,
{
    common: CommonReceivingRunner<SC, TC, ParachainSpecialClientApi<SC>>,
}

impl<SC, TC> BridgeParachainReceivingRunner<SC, TC>
where
    SC: S2SParaBridgeClientSolochain,
    TC: S2SClientRelay,
{
    pub fn new(input: MessageReceivingInput<SC, TC>, para_id: u32) -> Self {
        let different = ParachainSpecialClientApi {
            para_id,
            client: input.client_source.clone(),
        };
        let common = CommonReceivingRunner::new(input, different);
        Self { common }
    }

    pub async fn start(&self) -> RelayResult<()> {
        self.common.start().await
    }
}
