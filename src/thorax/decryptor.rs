use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

pub struct Decryptor {
    private_key: RsaPrivateKey,
}

impl Decryptor {
    pub fn new(private_key: &str) -> Self {
        Decryptor {
            private_key: RsaPrivateKey::from_pkcs1_pem(private_key)
                .expect("Failed to parse private key"),
        }
    }

    pub fn decrypt(&self, message: &[u8]) -> Vec<u8> {
        self.private_key
            .decrypt(Pkcs1v15Encrypt, message)
            .expect("failed to decrypt")
    }
}
