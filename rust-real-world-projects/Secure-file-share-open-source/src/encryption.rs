use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::fs;

pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Vec<u8> {
    public_key
        .encrypt(&mut OsRng, PaddingScheme::new_pkcs1v15_encrypt(), data)
        .expect("Encryption failed")
}

pub fn decrypt(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Vec<u8> {
    private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_data)
        .expect("Decryption failed")
}

pub fn save_keys_to_files() {
    let (private_key, public_key) = generate_keys();
    fs::write("keys/private_key.pem", private_key.to_pkcs1_pem().unwrap()).unwrap();
    fs::write("keys/public_key.pem", public_key.to_pkcs1_pem().unwrap()).unwrap();
}
