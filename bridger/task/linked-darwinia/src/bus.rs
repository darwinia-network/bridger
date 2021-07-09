use std::ops::Deref;

use lifeline::prelude::*;
use once_cell::sync::OnceCell;

static BUS: OnceCell<DarwiniaLinkedBus> = OnceCell::new();

pub(crate) fn bus() -> &'static DarwiniaLinkedBus {
    BUS.get_or_init(|| Default::default())
}

lifeline_bus!(pub struct DarwiniaLinkedBus);
