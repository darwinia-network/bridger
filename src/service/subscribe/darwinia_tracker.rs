use crate::error::Result;
use darwinia::Darwinia;
use std::time::Duration;
use tokio::time::delay_for;
use substrate_subxt::{
    Runtime,
    system::System,
};

/// DarwiniaTracker
pub struct DarwiniaBlockTracker<R: Runtime> {
	darwinia: Darwinia<R>,
	next_block: u32,
}

impl<R: Runtime> DarwiniaBlockTracker<R> {
	/// new
	pub fn new(darwinia: Darwinia<R>, scan_from: u32) -> Self {
		Self {
			darwinia,
			next_block: scan_from,
		}
	}

	/// get next block
	pub async fn next_block(&mut self) -> Result<<R as System>::Header>
        where R: System<BlockNumber = u32>,
    {
		loop {
			match self.get_next_block().await {
				Ok(result) => {
					if let Some(header) = result {
						return Ok(header);
					} else {
						delay_for(Duration::from_secs(6)).await;
					}
				}
				Err(err) => {
					if let Some(e) = err.downcast_ref::<substrate_subxt::Error>() {
						match e {
							substrate_subxt::Error::Rpc(
								jsonrpsee_types::error::Error::RestartNeeded(_),
							) => {
								return Err(crate::error::Error::RestartFromJsonrpsee.into());
							}
							_ => {
								error!(
									"An error occurred while tracking next darwinia block: {:#?}",
									e
								);
								delay_for(Duration::from_secs(30)).await;
							}
						}
					} else {
						error!(
							"An error occurred while tracking next darwinia block: {:#?}",
							err
						);
						delay_for(Duration::from_secs(30)).await;
					}
				}
			}
		}
	}

	async fn get_next_block(&mut self) -> Result<Option<<R as System>::Header>>
        where R: System<BlockNumber = u32>,
    {
		let finalized_block_hash = self.darwinia.finalized_head().await?;
		match self
			.darwinia
			.get_block_number_by_hash(finalized_block_hash)
			.await?
		{
			Some(finalized_block_number) => {
				if self.next_block > finalized_block_number {
					Ok(None)
				} else {
					let header = self.darwinia.get_block_by_number(self.next_block).await?;
					self.next_block += 1;
					Ok(Some(header))
				}
			}
			None => Ok(None),
		}
	}
}
