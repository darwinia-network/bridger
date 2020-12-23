//! tools
use crate::error::Result;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::error::Error;

/// Last redeemed ethereum block file name
pub const LAST_REDEEMED_CACHE_FILE_NAME: &str = "last-redeemed";

/// Last tracked darwinia block file name
pub const LAST_TRACKED_ETHEREUM_BLOCK_FILE_NAME: &str = "last-tracked-darwinia-block";

/// Get cache
pub async fn get_cache(data_dir: PathBuf, filename: &str, err: Error) -> Result<u64> {
    let mut filepath = data_dir;
    filepath.push(filename);

    // read from cache file
    match File::open(filepath).await {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await?;
            let cache = buffer.trim().parse()?;
            Ok(cache)
        },
        Err(_err) => {
            Err(err.into())
        }
    }
}

/// Set cache
pub async fn set_cache(data_dir: PathBuf, filename: &str, value: u64) -> Result<()> {
    let mut filepath = data_dir;
    filepath.push(filename);
    let mut file = File::create(filepath).await?;
    file.write_all(value.to_string().as_bytes()).await?;
    Ok(())
}
