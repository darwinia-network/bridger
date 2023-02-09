use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;
use support_common::config::{Config, Names};
use sysinfo::{ProcessExt, System, SystemExt};

use crate::config::BridgerConfig;
use support_common::error::BridgerError;
use support_terminal::output;

use crate::external;
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
        let (command, path_binary) = self.download_and_extract_binary(path, false)?;

        let cwd = path_binary.parent().map(|v| v.join("")).ok_or_else(|| {
            BridgerError::Subcommand("Can not get current binary's path".to_string())
        })?;
        external::provider::common::execute_binary(command, path_binary, self.args.clone(), cwd)
    }
}

impl PrecompiledBinaryExecutor {
    fn download_and_extract_binary(
        &self,
        path: Option<String>,
        force: bool,
    ) -> color_eyre::Result<(String, PathBuf)> {
        let config: BridgerConfig = Config::restore(Names::Bridger)?;
        let version = config
            .registry
            .version
            .unwrap_or_else(|| VERSION.to_string());
        for prefix in support_types::constants::ALLOW_BINARY_PREFIX {
            let command = format!("{}{}", prefix, self.command);

            output::output_text(format!("Try execute {}@{}", command, version));
            match self.download_and_extract_binary_with_command(
                path.clone(),
                &version,
                &command,
                force,
            ) {
                Ok(v) => return Ok((command, v)),
                Err(e) => {
                    output::output_err(format!("{e:?}"));
                }
            }
        }
        Err(BridgerError::UnsupportExternal(format!(
            "Not support this subcommand: {}",
            self.command
        ))
        .into())
    }

    fn download_and_extract_binary_with_command(
        &self,
        path: Option<String>,
        version: impl AsRef<str>,
        command: impl AsRef<str>,
        force: bool,
    ) -> color_eyre::Result<PathBuf> {
        let command = command.as_ref();
        let version = version.as_ref();
        // https://github.com/darwinia-network/bridger/releases/download/v0.4.11/bridger-x86_64-linux-gnu.tar.bz2
        let path = path.ok_or_else(|| {
            BridgerError::Subcommand("Missing remote base url for precompiled binary".to_string())
        })?;
        let package_name = self.package_name(command)?;
        let remote_url = format!("{}/releases/download/v{}/{}", path, version, package_name);
        let path_binary_base = std::env::current_exe()?
            .parent()
            .map(|v| v.join(""))
            .ok_or_else(|| {
                BridgerError::Subcommand("Can not get current binary path".to_string())
            })?;

        let path_binary = path_binary_base.join(if cfg!(windows) {
            format!("{}.exe", command)
        } else {
            command.to_string()
        });

        let path_download_package = path_binary_base.join(&package_name);

        tracing::trace!(target: "bridger", "Force mode: {}", force);
        if force && path_download_package.exists() {
            tracing::trace!(
                target: "bridger",
                "The download package is exists. remove it. {}",
                path_download_package.display()
            );
            std::fs::remove_file(&path_download_package)?;
        }

        if !force && path_binary.exists() {
            let version_output = Command::new(&path_binary).args(["--version"]).output()?;
            match version_output.status.code() {
                Some(0) => {
                    let stdout = String::from_utf8_lossy(&version_output.stdout).into_owned();
                    let parts: Vec<&str> = stdout.split(' ').collect();
                    let binary_version = parts.get(1);
                    if let Some(&bversion) = &binary_version {
                        if bversion.trim() == version.trim() {
                            return Ok(path_binary);
                        }
                    }
                    tracing::warn!(
                        target: "bridger",
                        "The expected version is [{}], but the binary's ({}) version is [{}].",
                        version,
                        command,
                        binary_version.unwrap_or(&"UNKNOWN")
                    );
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    for (pid, process) in sys.processes() {
                        if !process.name().contains(command) {
                            continue;
                        }
                        if cfg!(windows) {
                            return Err(BridgerError::Custom(format!(
                                "The {} is running (PID: {}), can not update to new version",
                                command, pid
                            ))
                            .into());
                        } else {
                            tracing::warn!(
                                target: "bridger",
                                "The binary ({}) will updated to {}, when finished, please restart your running progress.",
                                command,
                                version
                            );
                        }
                        break;
                    }
                    tracing::trace!(
                        target: "bridger",
                        "The version changed, remove old binary for command: {}",
                        command
                    );
                    std::fs::remove_file(&path_binary)?;
                }
                _ => {
                    let stderr = String::from_utf8_lossy(&version_output.stderr).into_owned();
                    return Err(BridgerError::Subcommand(format!(
                        "Can not get version from [{}]: {}",
                        path_binary.display(),
                        stderr
                    ))
                    .into());
                }
            }
        }

        tracing::trace!(
            target: "bridger",
            "Download package path is: {}",
            path_download_package.display(),
        );
        if !path_download_package.exists() {
            let mut url_package = remote_url.clone();
            let mut response;
            let mut times = 0;
            loop {
                times += 1;
                if times > 5 {
                    return Err(BridgerError::Custom(format!(
                        "Too many redirect times for download url: {}",
                        &remote_url
                    ))
                    .into());
                }
                output::output_text(format!("Downloading `{}`", url_package));
                response = reqwest::blocking::get(&url_package)?;
                let code = response.status().as_u16();
                tracing::trace!(target: "bridger", "Response code is: {}", code);
                let headers = response.headers();
                if let Some(value) = headers.get("Location") {
                    url_package = value.to_str()?.to_string();
                    tracing::trace!(target: "bridger", "Found redirect location: {}", &url_package);
                    continue;
                }
                break;
            }
            let code = response.status().as_u16();
            if code != 200 && code != 201 {
                return Err(BridgerError::Custom(format!(
                    "[{}] Failed to download package. the url is: {}",
                    code, remote_url
                ))
                .into());
            }
            let mut content = Cursor::new(response.bytes()?);
            let mut file = std::fs::File::create(&path_download_package)?;
            std::io::copy(&mut content, &mut file)?;
            output::output_text("Downloaded");
        }

        if force && path_binary.exists() {
            tracing::trace!(
                target: "bridger",
                "The binary file is exists. remove it. {}",
                path_binary.display()
            );
            std::fs::remove_file(&path_binary)?;
        }

        output::output_text(format!("Start extract {}", path_download_package.display()));
        let file = std::fs::File::open(&path_download_package)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut zip_inner_file = archive.by_index(i)?;
            let outpath = match zip_inner_file.enclosed_name() {
                Some(path) => path_binary_base.join(path),
                None => continue,
            };

            {
                let comment = zip_inner_file.comment();
                if !comment.is_empty() {
                    tracing::debug!(target: "bridger", "File {} comment: {}", i, comment);
                }
            }

            if zip_inner_file.name().ends_with('/') {
                tracing::debug!(target: "bridger", "File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(&outpath)?;
            } else {
                tracing::debug!(
                    target: "bridger",
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    zip_inner_file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut zip_inner_file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = zip_inner_file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                }
            }
        }

        std::fs::remove_file(&path_download_package)?;
        Ok(path_binary)
    }

    fn package_name(&self, base_name: impl AsRef<str>) -> color_eyre::Result<String> {
        let os = sys_info::os_type()?;
        let arch = if cfg!(target_arch = "x86") {
            "x86"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            return Err(BridgerError::Subcommand(
                "Can not support current architecture".to_string(),
            )
            .into());
        };
        let package_name = format!(
            "{}-{}-{}.{}",
            base_name.as_ref(),
            os.to_lowercase(),
            arch,
            &self.pkg_type
        );
        Ok(package_name)
    }
}
