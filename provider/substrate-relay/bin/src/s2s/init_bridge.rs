use crate::error;
use relay_substrate_client::Chain as RelaySubstrateChain;

pub struct InitBridge<SourceChain: RelaySubstrateChain, TargetChain: RelaySubstrateChain> {
    source: SourceChain,
    target: SourceChain,
    target_transactions_signer: TargetChain::AccountId,
}

impl<SourceChain: RelaySubstrateChain, TargetChain: RelaySubstrateChain>
    InitBridge<SourceChain, TargetChain>
{
    pub fn new(
        source: SourceChain,
        target: SourceChain,
        target_transactions_signer: TargetChain::AccountId,
    ) -> Self {
        Self {
            source,
            target,
            target_transactions_signer,
        }
    }
}

impl<SourceChain: RelaySubstrateChain, TargetChain: RelaySubstrateChain>
    InitBridge<SourceChain, TargetChain>
{
    pub fn run(self) -> error::Result<()> {
        Ok(())
    }
}
