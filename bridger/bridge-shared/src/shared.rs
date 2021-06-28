use lifeline::Service;

use bridge_standard::bridge::sand::BridgeSand;

use crate::bus::SharedBus;
use crate::channel::SharedChannel;
use crate::config::{DarwiniaServiceConfig, SharedConfig};
use crate::material::MaterialDarwinia;
use crate::service::darwinia::DarwiniaSharedService;
use crate::traits::{SharedKeep, SharedKeepService};

//// ---- it's ok
// #[derive(Debug)]
// pub struct BridgeShared {
//     config: SharedConfig,
//     bus: SharedBus,
//     // services: Vec<Box<dyn SharedKeepService<dyn SharedKeep>>>,
//     service_darwinia: DarwiniaSharedService<MaterialDarwinia>,
// }
//
// impl BridgeShared {
//     pub fn new(config: SharedConfig) -> anyhow::Result<Self> {
//         let bus = SharedBus::default();
//         let service_darwinia = DarwiniaSharedService::<MaterialDarwinia>::spawn(&bus)?;
//         Ok(BridgeShared {
//             config,
//             bus,
//             service_darwinia,
//         })
//     }
// }
//
// impl BridgeShared {
//     pub fn start(&mut self) -> anyhow::Result<()> {
//         self.config.store_darwinia::<MaterialDarwinia>()?;
//
//         // let darwinia_service = DarwiniaSharedService::<MaterialDarwinia>::spawn(&self.bus)?;
//         // self.services.push(Box::new(darwinia_service));
//         Ok(())
//     }
//
//     pub fn channel(&self) -> anyhow::Result<SharedChannel> {
//         Ok(SharedChannel::new())
//     }
// }

#[derive(Debug)]
pub struct BridgeShared {
    config: SharedConfig,
    bus: SharedBus,
    services: Vec<Box<dyn SharedKeepService<dyn SharedKeep>>>,
}

impl BridgeShared {
    pub fn new(config: SharedConfig) -> anyhow::Result<Self> {
        let bus = SharedBus::default();
        Ok(BridgeShared {
            config,
            bus,
            services: vec![],
        })
    }
}

impl BridgeShared {
    pub fn start(&mut self) -> anyhow::Result<()> {
        self.config.store_darwinia::<MaterialDarwinia>()?;

        let darwinia_service = DarwiniaSharedService::<MaterialDarwinia>::spawn(&self.bus)?;
        self.services.push(Box::new(darwinia_service));
        Ok(())
    }

    pub fn channel(&self) -> anyhow::Result<SharedChannel> {
        Ok(SharedChannel::new())
    }
}
