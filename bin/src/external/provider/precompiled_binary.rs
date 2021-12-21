use support_common::error::BridgerError;

use crate::external::execute::ISubcommandExecutor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub struct PrecompiledBinaryExecutor {
    command: String,
    args: Vec<String>,
    pkg_type: String,
}

impl PrecompiledBinaryExecutor {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            pkg_type: "zip".to_string(),
        }
    }
}

impl ISubcommandExecutor for PrecompiledBinaryExecutor {
    fn execute(&self, path: Option<String>) -> color_eyre::Result<()> {
        // https://github.com/darwinia-network/bridger/releases/download/v0.4.11/bridger-x86_64-linux-gnu.tar.bz2
        let path = path.ok_or(BridgerError::Subcommand(
            "Missing remote base url for precompiled binary".to_string(),
        ))?;
        let binary_name = self.binary_name(&self.command)?;
        let remote_url = format!("{}/releases/download/{}/{}", path, VERSION, binary_name);
        // todo: there need to download file and unzip then call it.
        println!("{}", remote_url);

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
        Ok(())
    }
}

impl PrecompiledBinaryExecutor {
    fn binary_name(&self, base_name: impl AsRef<str>) -> color_eyre::Result<String> {
        let os = sys_info::os_type()?;
        let arch = if cfg!(target_arch = "x86") {
            "x86"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            return Err(
                BridgerError::Subcommand("Can not support current arch".to_string()).into(),
            );
        };
        let binary_name = format!(
            "{}-{}-{}.{}",
            base_name.as_ref(),
            os.to_lowercase(),
            arch,
            &self.pkg_type
        );
        Ok(binary_name)
    }
}
