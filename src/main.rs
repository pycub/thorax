mod thorax;

use std::env;
use thorax::decryptor::Decryptor;
use thorax::encryptor::Encryptor;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    const PRIVATE_KEY: String = env::var("PRIVATE_KEY").unwrap();
    const PUBLIC_KEY: String = env::var("PUBLIC_KEY").unwrap();

    let message = "This is my very secret message!";

    let encrypted_message = Encryptor::new(&PUBLIC_KEY).encrypt(message);
    let decrypted_message = Decryptor::new(&PRIVATE_KEY).decrypt(&encrypted_message);

    assert_eq!(message.as_bytes(), decrypted_message);
}
