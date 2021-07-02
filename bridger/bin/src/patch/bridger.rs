use std::path::PathBuf;

pub fn is_allow_config_format<F: AsRef<str>>(format: F) -> bool {
    let vec = vec!["toml", "yml", "json"];
    vec.contains(&format.as_ref())
}

pub fn base_path(except_path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let base_path = except_path.unwrap_or_else(|| {
        let mut path = std::env::temp_dir();
        path.push("darwinia-bridger");
        path
    });
    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    Ok(base_path)
}
