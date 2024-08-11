use rand::Rng;
use sha2::{Sha256, Digest};
use std::fmt::Write;

fn generate_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key
}

fn key_to_hex(key: &[u8]) -> String {
    let mut hex = String::new();
    for byte in key.iter() {
        write!(&mut hex, "{:02x}", byte).unwrap();
    }
    hex
}

fn main() {
    let prefix = "616e7a61";
    let mut found = false;
    let mut count = 0;

    while !found {
        let key = generate_key();
        let hash = Sha256::digest(&key);
        let hash_slice = hash.as_slice(); 
        let hash_hex = key_to_hex(hash_slice);

        if hash_hex.starts_with(prefix) {
            println!("Found key: {}", hash_hex);
            found = true;
        }

        count += 1;
        if count % 10000 == 0 {
            println!("Checked {} keys...", count);
        }
    }
}
