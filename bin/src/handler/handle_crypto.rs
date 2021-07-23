use crate::types::command::CryptoCommand;

pub async fn handle_crypto(command: CryptoCommand) -> anyhow::Result<()> {
    match command {
        CryptoCommand::Encrypt { options } => {
            let passwd = rpassword::prompt_password_stdout("Please enter password:")?;
            let confirm = rpassword::prompt_password_stdout("Please confirm password:")?;
            if passwd != confirm {
                println!("Two passwords are inconsistent");
                return Ok(());
            }
            let crypto = bridge_primitives::crypto::Crypto::new();
            let encrypted = crypto.encrypt(&passwd, &options.value)?;
            println!("{}", encrypted);
        }
        CryptoCommand::Decrypt { options } => {
            let passwd = rpassword::prompt_password_stdout("Please enter password:")?;
            let crypto = bridge_primitives::crypto::Crypto::new();
            match crypto.decrypt(&passwd, &options.value) {
                Ok(decrypted) => println!("{}", decrypted),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    Ok(())
}
