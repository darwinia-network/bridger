use std::path::PathBuf;
use support_common::error::BridgerError;

pub fn list_externals(
    except_base_path: Option<PathBuf>,
) -> color_eyre::Result<(PathBuf, Vec<String>)> {
    let base_path = except_base_path
        .or(std::env::current_exe()?.parent().map(|v| v.join("")))
        .ok_or_else(|| {
            BridgerError::Custom("Can not get base path for external command".to_string())
        })?;
    tracing::trace!(target: "bridger", "The external base path is: {:?}", base_path);
    let read_dir = std::fs::read_dir(&base_path)?;
    let mut binaries = Vec::new();
    for dir in read_dir {
        let path = dir?.path();
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name() {
            Some(v) => v.to_string_lossy().to_string(),
            None => continue,
        };
        if !support_common::constants::ALLOW_BINARY_PREFIX
            .iter()
            .any(|&item| name.starts_with(item))
        {
            continue;
        }
        if &name == "bridger" || &name == "bridger.exe" {
            continue;
        }
        binaries.push(name);
    }
    Ok((base_path, binaries))
}
