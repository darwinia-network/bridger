use crate::error::BizError;
use crate::error::Result;
use crate::Crypto;
use rpassword::prompt_password_stdout;

/// Encrypt Key or Decrypt Key
pub async fn exec(key: String, is_decrypt: bool) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	let passwd = prompt_password_stdout("Please enter password:")?;

	let crypto = Crypto { salt: [0; 16] };
	if is_decrypt {
		let decrypted = crypto.decrypt(&passwd, &key)?;
		println!("{}", decrypted);
	} else {
		let confirmed = prompt_password_stdout("Please enter password again:")?;
		if passwd != confirmed {
			return Err(BizError::Bridger("Two passwords are inconsistent".to_string()).into());
		}
		let encrypted = crypto.encrypt(&passwd, &key)?;
		println!("{}", encrypted);
	}

	Ok(())
}
