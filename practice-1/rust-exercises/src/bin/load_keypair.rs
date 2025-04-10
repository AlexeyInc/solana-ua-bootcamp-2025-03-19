use bs58;
use dotenv::dotenv;
use solana_sdk::signature::{Keypair, Signer};
use std::env;
use std::fs;

fn main() {
    dotenv().ok();

    // Если хочешь использовать .env:
    let pk_json = env::var("PK").unwrap_or_else(|_| {
        fs::read_to_string("keypair.json").expect("Failed to read keypair.json")
    });

    let key_bytes: Vec<u8> = serde_json::from_str(&pk_json).expect("Invalid JSON in PK");
    let secret: [u8; 64] = key_bytes
        .try_into()
        .expect("Keypair must be exactly 64 bytes");
    let keypair = Keypair::from_bytes(&secret).expect("Invalid keypair");

    println!("Loaded Keypair!");
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
    println!(
        "🗝️ Base58 Private Key для Phantom: {}",
        bs58::encode(keypair.to_bytes()).into_string()
    );
}
