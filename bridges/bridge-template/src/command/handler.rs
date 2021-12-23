use lifeline::{Bus, Sender};

use support_terminal::types::BasicOptions;

use crate::bridge::{TemplateTask, TemplateTaskMessage};

pub async fn handle_start(_basic_options: BasicOptions) -> color_eyre::Result<()> {
    tracing::info!("Start bridge template");
    let task = TemplateTask::new()?;
    let stack = task.stack();
    let bus = stack.bus();

    let mut sender = bus.tx::<TemplateTaskMessage>()?;
    let mut times = 0;
    loop {
        times += 1;
        if times > 10 {
            sender.send(TemplateTaskMessage::StopSomeService).await?;
            return Ok(());
        }
        sender.send(TemplateTaskMessage::SomeEvent).await?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
