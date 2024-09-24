mod thorax;

use thorax::decryptor::Decryptor;
use thorax::encryptor::Encryptor;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    const PRIVATE_KEY: String = env::var("PRIVATE_KEY").unwrap();
    const PUBLIC_KEY: String = env::var("PUBLIC_KEY").unwrap();

    // build our application with a route
    // let app = Router::new();

    let message = "This is my very secret message!";

    let encrypted_message = Encryptor::new(&PUBLIC_KEY).encrypt(message);
    let decrypted_message = Decryptor::new(&PRIVATE_KEY).decrypt(&encrypted_message);

    assert_eq!(message.as_bytes(), decrypted_message);

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
