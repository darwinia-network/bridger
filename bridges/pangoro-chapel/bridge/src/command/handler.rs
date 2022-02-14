use lifeline::{Bus, Sender};

use crate::bridge::{PangoroChapelTask, TemplateTaskMessage};

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge template");
    let task = PangoroChapelTask::new()?;
    let stack = task.stack();
    let bus = stack.bus();

    let mut sender = bus.tx::<TemplateTaskMessage>()?;
    let mut times = 0;
    loop {
        times += 1;
        if times > 10 {
            sender.send(TemplateTaskMessage::StopSomeService).await?;
            tracing::info!("Execute success");
            return Ok(());
        }
        sender.send(TemplateTaskMessage::SomeEvent(times)).await?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
