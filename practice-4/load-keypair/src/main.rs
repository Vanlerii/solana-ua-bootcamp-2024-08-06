use dotenv::dotenv;
use solana_sdk::signature::{Keypair, Signer};
use serde_json;
use std::env;
use bs58;

fn main() {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");


    let secret_key_bytes: Vec<u8> = serde_json::from_str(&private_key)
        .expect("Failed to parse SECRET_KEY from JSON");

    let keypair = Keypair::from_bytes(&secret_key_bytes)
        .expect("Failed to create Keypair from secret key");

    let public_key_base58 = bs58::encode(keypair.pubkey().as_ref()).into_string();
    println!("Public key: {}", public_key_base58);
}
