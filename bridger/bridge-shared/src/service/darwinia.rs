use std::marker::PhantomData;

use lifeline::{Bus, Lifeline, Service, Task};

use bridge_config::config::component::BeeConfig;
use bridge_config::Config;
use bridge_standard::bridge::chain::LikeDarwiniaChain;
use bridge_standard::bridge::config::BridgeConfig;
use bridge_standard::bridge::sand::BridgeSand;

use crate::bus::SharedBus;
use crate::traits::{SharedKeep, SharedKeepService, SharedMaterial};

#[derive(Debug)]
pub struct DarwiniaSharedService<M: SharedMaterial> {
    _greet: Lifeline,
    _marker: PhantomData<fn() -> M>,
}

impl<M: SharedMaterial> SharedKeepService<M> for DarwiniaSharedService<M> {}

impl<M: SharedMaterial> lifeline::Service for DarwiniaSharedService<M>
where
    M::Chain: LikeDarwiniaChain,
{
    type Bus = M::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(&format!("{}-extrinsic", M::NAME), async move {
            let config: BeeConfig = Config::restore(M::NAME)?;
            debug!("shared bee config: {:?}", config);
            Ok(())
        });
        Ok(Self {
            _greet,
            _marker: Default::default(),
        })
    }
}
