use anyhow::Result;
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};

use crate::config::Config;

fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// Gets the keypair pointed by the configuration `keys.location`.
/// 
/// If a keypair doesn't exist, it generates a new one.
pub fn get_keypair(config: &Config) -> Result<(RsaPrivateKey, RsaPublicKey)> {
    let loc = &config.keys.location;

    let pubpath = format!("{}/pub", loc);
    let privpath = format!("{}/priv", loc);

    if !std::path::Path::new(&pubpath).exists() || !std::path::Path::new(&privpath).exists() {
        let (private_key, public_key) = generate_keys();

        std::fs::write(&pubpath, public_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)?).expect("Failed to write the public key");
        std::fs::write(&privpath, private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)?).expect("Failed to write the private key");

        Ok((private_key, public_key))
    } else {
        let private_key = RsaPrivateKey::from_pkcs1_pem(&std::fs::read_to_string(privpath).expect("Failed to read the private key")).expect("Failed to parse the private key");
        let public_key = RsaPublicKey::from_pkcs1_pem(&std::fs::read_to_string(pubpath).expect("Failed to read the public key")).expect("Failed to parse the public key");

        Ok((private_key, public_key))
    }
}