use std::collections::HashMap;

use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{ChaChaRng, CryptoRng, FromEntropy, RngCore};

type Cipher = Cbc<Aes256, Pkcs7>;
const IV_LENGTH: usize = 16;
const KEY_LENGTH: usize = 32;
const AES_PREFERENCES_KEY: &str = "crypto_key";

#[derive(Debug)]
pub enum CryptoError {
    Failed,
    WrongKeyLength,
}

pub struct Crypto {
    key: Vec<u8>,
    rng: Box<dyn RngCore>,
}

impl Crypto {
    pub fn from_storage(storage: &mut HashMap<String, String>) -> Result<Self, CryptoError> {
        let mut rng = ChaChaRng::from_entropy();

        let mut key = if let Some(key) = storage.get(AES_PREFERENCES_KEY) {
            base64::decode(key).unwrap_or_default()
        } else {
            Vec::new()
        };
        if key.len() != KEY_LENGTH {
            key.resize(KEY_LENGTH, 0);
            rng.fill_bytes(&mut key);
            storage.clear();
            storage.insert(String::from(AES_PREFERENCES_KEY), base64::encode(&key));
        }

        Self::new(&key, rng)
    }

    pub fn new<R>(key: &[u8], rng: R) -> Result<Self, CryptoError>
    where
        R: CryptoRng + RngCore + 'static,
    {
        if key.len() != KEY_LENGTH {
            return Err(CryptoError::WrongKeyLength);
        }
        Ok(Self {
            key: key.to_vec(),
            rng: Box::new(rng),
        })
    }

    pub fn encrypt(&mut self, data: &str) -> Result<String, CryptoError> {
        let mut iv = [0 as u8; IV_LENGTH];
        self.rng.fill_bytes(&mut iv);
        let cipher = Cipher::new_var(&self.key, &iv).map_err(|_err| CryptoError::Failed)?;
        let mut encrypted = cipher.encrypt_vec(data.as_bytes());

        let mut full_data = Vec::with_capacity(encrypted.len() + IV_LENGTH);
        full_data.extend_from_slice(&iv);
        full_data.append(&mut encrypted);

        Ok(base64::encode(&full_data))
    }

    pub fn decrypt(&self, data: &str) -> Result<String, CryptoError> {
        let full_data = base64::decode(data).map_err(|_err| CryptoError::Failed)?;
        let iv = &full_data[0..IV_LENGTH];
        let encrypted = &full_data[IV_LENGTH..];
        let cipher = Cipher::new_var(&self.key, iv).map_err(|_err| CryptoError::Failed)?;
        let decrypted = cipher
            .decrypt_vec(encrypted)
            .map_err(|_err| CryptoError::Failed)?;
        Ok(String::from_utf8(decrypted).map_err(|_err| CryptoError::Failed)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{Crypto, KEY_LENGTH};

    use rand::{ChaChaRng, FromEntropy, RngCore};

    #[test]
    fn test_crypto() {
        let mut key = [0 as u8; KEY_LENGTH];
        let mut rng = ChaChaRng::from_entropy();
        rng.fill_bytes(&mut key);
        let mut crypto = Crypto::new(&key, rng).expect("Cannot create crypto");

        let plaintext = "This is plain text.";
        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(
            plaintext, decrypted,
            "Decrypted text doesn't match plaintext"
        );
    }
}
