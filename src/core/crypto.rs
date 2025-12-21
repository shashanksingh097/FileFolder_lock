use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use argon2::Argon2;
use rand::{rngs::OsRng, RngCore};

pub fn encrypt(data: &[u8], password: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let argon2 = Argon2::default();
    let mut key_bytes = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut key_bytes)
        .expect("Key derivation failed");

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted = cipher.encrypt(nonce, data).expect("Encrypt failed");

    (salt.to_vec(), nonce_bytes.to_vec(), encrypted)
}

pub fn decrypt(
    encrypted: &[u8],
    password: &str,
    salt: &[u8],
    nonce: &[u8],
) -> Result<Vec<u8>, ()> {
    let argon2 = Argon2::default();
    let mut key_bytes = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key_bytes)
        .map_err(|_| ())?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    cipher.decrypt(Nonce::from_slice(nonce), encrypted).map_err(|_| ())
}
