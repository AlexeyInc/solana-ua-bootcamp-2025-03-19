use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use std::env;

fn main() {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let pk_json = env::var("PK").expect("No PK in .env");
    let key_bytes: Vec<u8> = serde_json::from_str(&pk_json).expect("Invalid PK");
    let secret: [u8; 64] = key_bytes
        .try_into()
        .expect("Keypair must be exactly 64 bytes");
    let keypair = solana_sdk::signature::Keypair::from_bytes(&secret).unwrap();

    let pubkey = keypair.pubkey();
    println!("Checking balance for {}", pubkey);

    match client.get_balance(&pubkey) {
        Ok(balance) => println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0),
        Err(e) => eprintln!("Failed to get balance: {}", e),
    }
}
