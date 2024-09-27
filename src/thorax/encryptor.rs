use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::path::Path;

pub struct Encryptor {
    pub public_key: RsaPublicKey,
}

impl Encryptor {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Encryptor {
            public_key: RsaPublicKey::read_public_key_pem_file(path)
                .expect("Failed to parse public key"),
        }
    }

    pub fn encrypt(&self, message: &str) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        self.public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())
            .expect("failed to encrypt")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thorax::decryptor::Decryptor;
    use std::env;
    use dotenv::dotenv;

    #[test]
    fn test_encryption() {
        dotenv().ok();

        let encryptor: Encryptor = Encryptor::new(env::var("PUBLICKEY_PATH").unwrap());
        let decryptor: Decryptor = Decryptor::new(env::var("PRIVATEKEY_PATH").unwrap());

        let text = "fuck";

        let encrypted_text = encryptor.encrypt(text);
        let decrypted_text = decryptor.decrypt(encrypted_text.as_slice());

        assert_eq!(decrypted_text, text.bytes().collect::<Vec<u8>>());
    }
}