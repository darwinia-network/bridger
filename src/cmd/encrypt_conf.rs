use crate::error::Result;
use rpassword::prompt_password_stdout;
use crate::error::BizError;
use std::path::PathBuf;
use crate::Settings;
use std::fs;

/// Encrypt Key or Decrypt Key
pub async fn exec(from_path: String, to_path: String) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
    let from_path = PathBuf::from(from_path);
	let mut config = Settings::new(&from_path)?;
    let passwd = prompt_password_stdout("Please enter password:")?;
    if config.encrypted {
        config.decrypt(&passwd)?;
    } else {
        let confirmed = prompt_password_stdout("Please enter password again:")?;
        if passwd != confirmed {
            return Err(BizError::Bridger("Two passwords are inconsistent".to_string()).into())
        }
        config.encrypt(&passwd)?;
    }
    let new_config = serde_yaml::to_string(&config)?;
    fs::write(to_path + "/config.yml", new_config)?;
	Ok(())
}
