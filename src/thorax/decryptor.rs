use rsa::pkcs8::DecodePrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::path::Path;

pub struct Decryptor {
    private_key: RsaPrivateKey,
}

impl Decryptor {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Decryptor {
            private_key: RsaPrivateKey::read_pkcs8_pem_file(path)
                .expect("failed to parse private key"),
        }
    }

    pub fn decrypt(&self, message: &[u8]) -> Vec<u8> {
        self.private_key
            .decrypt(Pkcs1v15Encrypt, message)
            .expect("failed to decrypt")
    }
}
