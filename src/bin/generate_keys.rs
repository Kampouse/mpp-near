// Simple keypair generator for NEAR
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new signing key using ed25519-dalek 2.0 API
    let signing_key = SigningKey::generate(&mut OsRng);

    // Encode as NEAR format (ed25519:base58)
    let secret_bytes = signing_key.as_bytes();
    let verifying_key = signing_key.verifying_key();
    let public_bytes = verifying_key.as_bytes();

    // NEAR uses base58 encoding with "ed25519:" prefix
    let secret_b58 = bs58::encode(secret_bytes).into_string();
    let public_b58 = bs58::encode(public_bytes).into_string();

    println!("=== NEAR Account Credentials ===");
    println!();
    println!("Public Key (use for account creation):");
    println!("ed25519:{}", public_b58);
    println!();
    println!("Private Key (save this securely!):");
    println!("ed25519:{}", secret_b58);
    println!();
    println!("=== Account Creation Instructions ===");
    println!();
    println!("1. Choose an account name (e.g., myaccount.testnet)");
    println!("2. Create the account using one of these methods:");
    println!();
    println!("   Option A - Using NEAR CLI (install first):");
    println!("   near create-account myaccount.testnet --publicKey ed25519:{} --useFaucet", public_b58);
    println!();
    println!("   Option B - Using NEAR Wallet:");
    println!("   1. Go to https://wallet.testnet.near.org");
    println!("   2. Click 'Create Account'");
    println!("   3. Use the public key above");
    println!();
    println!("   Option C - Using existing account to fund:");
    println!("   near create-account myaccount.testnet --masterAccount existing.testnet --publicKey ed25519:{}", public_b58);
    println!();
    println!("IMPORTANT: Save your private key securely!");
    println!("Do not share it with anyone!");

    Ok(())
}
