//! Tool to show the public key for a given private key

use clap::Parser;
use near_crypto::SecretKey;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    private_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let secret_key = SecretKey::from_str(&cli.private_key)?;

    let public_key = secret_key.public_key();

    println!("Private key: {}", cli.private_key);
    println!("Public key:  {}", public_key);

    Ok(())
}
