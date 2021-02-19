use crate::error::{BizError, Result};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::scrypt::scrypt;
use crypto::scrypt::ScryptParams;
use crypto::{aes, blockmodes, buffer};

pub trait EncryptPrivateKey {
	fn encrypt(&mut self, crypto: &Crypto, passwd: &str) -> Result<()>;
	fn decrypt(&mut self, crypto: &Crypto, passwd: &str) -> Result<()>;
}

/// crypto for private key
pub struct Crypto {
	/// salt
	pub salt: [u8; 16],
}

impl Crypto {
	fn aes256_cbc_encrypt(&self, input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
		let mut encryptor =
			aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

		let mut encrypted = Vec::<u8>::new();
		let mut read_buffer = buffer::RefReadBuffer::new(input);
		let mut buffer = [0; 4096];
		let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

		loop {
			let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
			encrypted.extend(
				write_buffer
					.take_read_buffer()
					.take_remaining()
					.iter()
					.copied(),
			);

			match result {
				Err(_) => return Err(BizError::Bridger("Encryption failed".to_string()).into()),
				Ok(BufferResult::BufferUnderflow) => break,
				Ok(BufferResult::BufferOverflow) => {}
			}
		}
		Ok(encrypted)
	}

	fn aes256_cbc_decrypt(&self, input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
		let mut decryptor =
			aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

		let mut decrypted = Vec::<u8>::new();
		let mut read_buffer = buffer::RefReadBuffer::new(input);
		let mut buffer = [0; 4096];
		let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

		loop {
			let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
			decrypted.extend(
				write_buffer
					.take_read_buffer()
					.take_remaining()
					.iter()
					.copied(),
			);
			match result {
				Err(e) => {
					error!("[{:?}] please check your password", e);
					return Err(BizError::Bridger("Decryption failed".to_string()).into());
				}
				Ok(BufferResult::BufferUnderflow) => break,
				Ok(BufferResult::BufferOverflow) => {}
			}
		}

		Ok(decrypted)
	}

	fn generate_key(&self, passwd: &str) -> Result<[u8; 48]> {
		//let salt = [0; 16];
		let mut output = [0; 48];
		scrypt(
			&passwd.as_bytes(),
			&self.salt,
			&ScryptParams::new(10, 2, 3),
			&mut output,
		);
		Ok(output)
	}

	/// encrypt
	pub fn encrypt(&self, passwd: &str, plain: &str) -> Result<String> {
		let private_key = self.generate_key(&passwd)?;
		let key = &private_key[..32];
		let iv = &private_key[32..48];
		let encrypted = self.aes256_cbc_encrypt(plain.as_bytes(), &key, &iv)?;
		Ok(base64::encode(encrypted.as_slice()))
	}

	/// decrypt
	pub fn decrypt(&self, passwd: &str, encrypted: &str) -> Result<String> {
		let private_key = self.generate_key(&passwd)?;
		let key = &private_key[..32];
		let iv = &private_key[32..48];
		let encrypted_data = base64::decode(&encrypted)?;
		let decrypted_data = self.aes256_cbc_decrypt(&encrypted_data[..], &key, &iv)?;
		Ok(String::from_utf8(decrypted_data)?)
	}
}

/// impl Encrypt PrivateKey method encrypt and decrypt
#[macro_export]
macro_rules! encrypt_key {
	($name: ident) => {
		impl EncryptPrivateKey for $name {
			fn encrypt(&mut self, crypto: &Crypto, passwd: &str) -> Result<()> {
				self.private_key = crypto.encrypt(&passwd, &self.private_key)?;
				Ok(())
			}
			fn decrypt(&mut self, crypto: &Crypto, passwd: &str) -> Result<()> {
				self.private_key = crypto.decrypt(&passwd, &self.private_key)?;
				Ok(())
			}
		}
	};
}

#[test]
fn test_encrypt() {
	let crypto = Crypto { salt: [1; 16] };
	let plain = "Hello World!";
	let passwd = "123456";
	let encrypted = crypto.encrypt(&passwd, plain).unwrap();
	println!("{}", encrypted);
	let decrypted = crypto.decrypt(&passwd, &encrypted).unwrap();
	println!("{}", decrypted);
	assert_eq!(decrypted, plain);
}
