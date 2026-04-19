use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Context, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};

use crate::state::AuthState;

fn encryption_key(state: &AuthState) -> anyhow::Result<[u8; 32]> {
    let bytes = STANDARD
        .decode(state.admin_totp_encryption_key.as_ref())
        .context("invalid ADMIN_TOTP_ENCRYPTION_KEY encoding")?;

    let key: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow!("ADMIN_TOTP_ENCRYPTION_KEY must decode to 32 bytes"))?;

    Ok(key)
}

pub fn encrypt_totp_secret(state: &AuthState, secret: &str) -> anyhow::Result<String> {
    let key = encryption_key(state)?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("failed to initialize TOTP cipher")?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, secret.as_bytes())
        .map_err(|_| anyhow!("failed to encrypt TOTP secret"))?;

    let mut payload = nonce.to_vec();
    payload.extend_from_slice(&ciphertext);
    Ok(STANDARD.encode(payload))
}

pub fn decrypt_totp_secret(state: &AuthState, encrypted_secret: &str) -> anyhow::Result<String> {
    let key = encryption_key(state)?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("failed to initialize TOTP cipher")?;
    let payload = STANDARD
        .decode(encrypted_secret)
        .context("invalid encrypted TOTP payload")?;

    if payload.len() < 12 {
        return Err(anyhow!("encrypted TOTP payload is too short"));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let plaintext = cipher
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|_| anyhow!("failed to decrypt TOTP secret"))?;

    String::from_utf8(plaintext).context("decrypted TOTP secret is not valid UTF-8")
}
