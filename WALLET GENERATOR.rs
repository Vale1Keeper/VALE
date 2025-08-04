use rand::rngs::OsRng;
use ring::signature;
use ring::signature::KeyPair;
use sha2::{Sha256, Digest};
use std::fs;

fn main() {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    let public_key = key_pair.public_key().as_ref();

    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let hash = hasher.finalize();
    let address = format!("VLE{}", &hex::encode(&hash)[..39]);

    let output = format!(
        "VALE WALLET\n\nADDRESS:\n{}\n\nPRIVATE KEY:\n{}\n\nNEVER SHARE THIS",
        address,
        hex::encode(&pkcs8_bytes)
    );

    fs::write("wallet.txt", output).expect("Failed to save!");
    println!("\n✅ Wallet created!");
    println!("🔐 Address: {}", address);
    println!("📁 Saved to: wallet.txt");
}