use crate::command::output;
use crate::external;
use std::io::Cursor;
use std::path::PathBuf;
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
        // get path of binary
        let path_binary = self.path_binary(path)?;

        let cwd = path_binary
            .parent()
            .map(|v| v.join(""))
            .ok_or(BridgerError::Subcommand(
                "Can not get current binary path".to_string(),
            ))?;
        external::provider::common::execute_binary(
            self.command.clone(),
            path_binary,
            self.args.clone(),
            cwd,
        )
    }
}

impl PrecompiledBinaryExecutor {
    fn path_binary(&self, path: Option<String>) -> color_eyre::Result<PathBuf> {
        // https://github.com/darwinia-network/bridger/releases/download/v0.4.11/bridger-x86_64-linux-gnu.tar.bz2
        let path = path.ok_or(BridgerError::Subcommand(
            "Missing remote base url for precompiled binary".to_string(),
        ))?;
        let pakcage_name = self.pakcage_name(&self.command)?;
        let remote_url = format!("{}/releases/download/{}/{}", path, VERSION, pakcage_name);
        let path_binary_base = std::env::current_exe()?
            .parent()
            .map(|v| v.join(""))
            .ok_or(BridgerError::Subcommand(
                "Can not get current binary path".to_string(),
            ))?;

        let path_binary = path_binary_base.join(if cfg!(windows) {
            format!("{}.exe", &self.command)
        } else {
            self.command.clone()
        });
        if path_binary.exists() {
            return Ok(path_binary);
        }

        let download_pkg_name = path_binary_base.join(&pakcage_name);
        let mut file = std::fs::File::create(&download_pkg_name)?;

        if !download_pkg_name.exists() {
            output::output_text(format!("Downloading `{}`", remote_url));
            let response = reqwest::blocking::get(remote_url)?;
            let mut content = Cursor::new(response.bytes()?);
            std::io::copy(&mut content, &mut file)?;
            output::output_text("Downloaded");
        }

        output::output_text(format!("Start extract {}", pakcage_name));
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    tracing::debug!("File {} comment: {}", i, comment);
                }
            }

            if (&*file.name()).ends_with('/') {
                tracing::debug!("File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(&outpath)?;
            } else {
                tracing::debug!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                }
            }
        }
        Ok(path_binary)
    }

    fn pakcage_name(&self, base_name: impl AsRef<str>) -> color_eyre::Result<String> {
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
        let pakcage_name = format!(
            "{}-{}-{}.{}",
            base_name.as_ref(),
            os.to_lowercase(),
            arch,
            &self.pkg_type
        );
        Ok(pakcage_name)
    }
}
