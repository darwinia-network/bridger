use std::path::PathBuf;

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
