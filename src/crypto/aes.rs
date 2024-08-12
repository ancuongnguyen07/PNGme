use aes_gcm::aead::Aead;
use aes_gcm::{aead::OsRng, AeadCore, Aes256Gcm, Key, KeyInit};

use crate::Error;
use crate::Result;

fn aes256gcm_cipher(key_raw: &[u8]) -> Result<(Aes256Gcm)> {
    if key_raw.len() != 32 {
        return Err(Error::InvalidKeyLength);
    }

    let key_raw: [u8; 32] = key_raw.try_into().map_err(|_| Error::InvalidKey)?;

    // 256-bit key
    let key = Key::<Aes256Gcm>::from_slice(&key_raw);
    let cipher = Aes256Gcm::new(key);

    Ok(cipher)
}

/// Encrypt the given plaintext using the given key.
/// Return the ciphertext and Nonce used for encryption.
///
/// **Output format (`ciphertext`, `nonce`).**
pub fn aes256gcm_encrypt(data: &[u8], key_raw: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let cipher = aes256gcm_cipher(key_raw)?;
    // 96-bit nounce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data)
        .map_err(|_| Error::EncryptionErr)?;

    Ok((ciphertext, nonce.as_slice().to_vec()))
}

pub fn aes256gcm_decrypt(ciphertext: &[u8], key_raw: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = aes256gcm_cipher(key_raw)?;
    let plaintext = cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|_| Error::DecryptionErr)?;

    Ok(plaintext)
}
