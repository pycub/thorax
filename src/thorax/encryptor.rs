use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

pub struct Encryptor {
    public_key: RsaPublicKey,
}

impl Encryptor {
    pub fn new(public_key: &str) -> Self {
        Encryptor {
            public_key: RsaPublicKey::from_public_key_pem(public_key)
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
