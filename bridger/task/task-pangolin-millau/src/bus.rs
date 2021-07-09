use std::ops::Deref;

use lifeline::prelude::*;
use once_cell::sync::OnceCell;

lifeline_bus!(pub struct PangolinMillauBus);

// trait KeepBus {
//     fn as_any(&self) -> &dyn std::any::Any;
// }
//
// impl KeepBus for PangolinMillauBus {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
// }
//
// static BUS: OnceCell<Box<dyn KeepBus + Send + Sync>> = OnceCell::new();
//
// pub(crate) fn bus() -> &'static PangolinMillauBus {
//     BUS.get_or_init(|| Box::new(PangolinMillauBus::default()))
//         .as_any()
//         .downcast_ref::<PangolinMillauBus>()
//         .unwrap()
// }

// static BUS: OnceCell<Box<dyn lifeline::Bus + Send + Sync>> = OnceCell::new();
//
// pub(crate) fn bus() -> &'static PangolinMillauBus {
//     BUS.get_or_init(|| Box::new(Default::default()))
// }

static BUS: OnceCell<PangolinMillauBus> = OnceCell::new();

pub(crate) fn bus() -> &'static PangolinMillauBus {
    BUS.get_or_init(|| Default::default())
}
