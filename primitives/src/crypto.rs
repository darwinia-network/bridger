use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::scrypt::{scrypt, ScryptParams};
use crypto::{aes, blockmodes, buffer};

use crate::error::{BridgeBasicError, BridgeBasicResult};

/// crypto for private key
pub struct Crypto {
    /// salt
    salt: [u8; 16],
}

impl Default for Crypto {
    fn default() -> Self {
        Self::new()
    }
}

impl Crypto {
    pub fn new() -> Self {
        Self::new_with_salt([0; 16])
    }

    pub fn new_with_salt(salt: [u8; 16]) -> Self {
        Self { salt }
    }
}

impl Crypto {
    fn aes256_cbc_encrypt(
        &self,
        input: &[u8],
        key: &[u8],
        iv: &[u8],
    ) -> BridgeBasicResult<Vec<u8>> {
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
                Err(_) => return Err(BridgeBasicError::Crypto("Encryption failed".to_string())),
                Ok(BufferResult::BufferUnderflow) => break,
                Ok(BufferResult::BufferOverflow) => {}
            }
        }
        Ok(encrypted)
    }

    fn aes256_cbc_decrypt(
        &self,
        input: &[u8],
        key: &[u8],
        iv: &[u8],
    ) -> BridgeBasicResult<Vec<u8>> {
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
                    log::error!("[{:?}] please check your password", e);
                    return Err(BridgeBasicError::Crypto("Decryption failed".to_string()));
                }
                Ok(BufferResult::BufferUnderflow) => break,
                Ok(BufferResult::BufferOverflow) => {}
            }
        }

        Ok(decrypted)
    }

    fn generate_key(&self, passwd: &str) -> [u8; 48] {
        //let salt = [0; 16];
        let mut output = [0; 48];
        scrypt(
            &passwd.as_bytes(),
            &self.salt,
            &ScryptParams::new(10, 2, 3),
            &mut output,
        );
        output
    }

    /// encrypt
    pub fn encrypt(&self, passwd: &str, plain: &str) -> BridgeBasicResult<String> {
        let private_key = self.generate_key(&passwd);
        let key = &private_key[..32];
        let iv = &private_key[32..48];
        let encrypted = self.aes256_cbc_encrypt(plain.as_bytes(), &key, &iv)?;
        Ok(base64::encode(encrypted.as_slice()))
    }

    /// decrypt
    pub fn decrypt(&self, passwd: &str, encrypted: &str) -> BridgeBasicResult<String> {
        let private_key = self.generate_key(&passwd);
        let key = &private_key[..32];
        let iv = &private_key[32..48];
        let encrypted_data = base64::decode(&encrypted).map_err(|e| {
            BridgeBasicError::Crypto(format!("failed to decrypt, base64 decode error: {:?}", e))
        })?;
        let decrypted_data = self.aes256_cbc_decrypt(&encrypted_data[..], &key, &iv)?;
        String::from_utf8(decrypted_data).map_err(|e| {
            BridgeBasicError::Crypto(format!("failed to decrypt data to string: {:?}", e))
        })
    }
}
