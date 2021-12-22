use lifeline::prelude::*;
use lifeline::{Receiver, Sender};

use crate::message::ToTemplateLinkedMessage;
use crate::task::TemplateTask;

lifeline_bus!(pub struct TemplateTaskBus);
