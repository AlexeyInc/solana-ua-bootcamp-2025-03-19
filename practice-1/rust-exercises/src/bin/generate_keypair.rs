use solana_sdk::signature::{Keypair, Signer};
use std::fs::File;
use std::io::Write;

fn main() {
    let keypair = Keypair::new();
    let keypair_bytes = keypair.to_bytes();
    let serialized = serde_json::to_string(&keypair_bytes.to_vec()).unwrap();

    let mut file = File::create("keypair.json").expect("Failed to create file");
    file.write_all(serialized.as_bytes())
        .expect("Failed to write");

    println!("Public Key: {}", keypair.pubkey());
}
