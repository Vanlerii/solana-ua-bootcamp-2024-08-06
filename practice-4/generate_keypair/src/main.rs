use solana_sdk::signature::{Keypair, Signer};
use base64::{engine::general_purpose, Engine};

fn main() {

    let keypair = Keypair::new();
    

    let public_key_str = keypair.pubkey().to_string();
    
    let secret_key_str = base64::encode(&keypair.secret());
    
  
    println!("Generate keypair!");
    println!("Public key: {}", public_key_str);
    println!("Private key: {}", secret_key_str);
}
