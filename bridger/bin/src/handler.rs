use crate::types::command::TaskCommand;

pub fn handle_task_command(command: TaskCommand) -> anyhow::Result<()> {
    match command {
        TaskCommand::List => {}
        TaskCommand::Start => {}
        TaskCommand::Stop => {}
    };
    Ok(())
}
