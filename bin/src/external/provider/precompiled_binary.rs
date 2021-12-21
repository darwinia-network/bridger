use crate::external::execute::ISubcommandExecutor;

#[derive(Clone, Debug)]
pub struct PrecompiledBinaryExecutor {}

impl PrecompiledBinaryExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISubcommandExecutor for PrecompiledBinaryExecutor {
    fn execute(&self, path: Option<String>) -> color_eyre::Result<()> {
        // let (base_path, binaries) = external::helpers::list_externals(None)?;
        // if binaries.contains(&self.command) {}
        // if !binaries.contains(&self.command) {
        //     return Err(BridgerError::Subcommand(format!(
        //         "The command `{}` not found",
        //         &self.command.blue()
        //     ))
        //         .into());
        // }
        // println!("{} {:?}", self.command, self.args);
        // Ok(())
        todo!()
    }
}
