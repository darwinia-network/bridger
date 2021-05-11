// use crate::error;
//
// pub struct InitBridge<
//     SourceChain: relay_substrate_client::Chain,
//     TargetChain: relay_substrate_client::Chain,
// > {
//     source: SourceChain,
//     target: SourceChain,
//     target_transactions_signer: TargetChain::AccountId,
// }
//
// impl<SourceChain, TargetChain> InitBridge<SourceChain, TargetChain> {
//     pub fn new(
//         source: SourceChain,
//         target: SourceChain,
//         target_transactions_signer: TargetChain::AccountId,
//     ) -> Self {
//         Self {
//             source,
//             target,
//             target_transactions_signer,
//         }
//     }
// }
//
// impl<SourceChain, TargetChain> InitBridge<SourceChain, TargetChain> {
//     pub fn run(self) -> error::Result<()> {
//         Ok(())
//     }
// }
