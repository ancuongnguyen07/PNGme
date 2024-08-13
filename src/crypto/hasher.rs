use crate::Result;

use sha3::{Digest, Sha3_256};

/// SHA3: Hash the given text into 256-bit random bytes.
pub fn sha3_hash(text: &str) -> Result<Vec<u8>> {
    // Hash the passphrase
    let mut hasher = Sha3_256::new();
    hasher.update(text.as_bytes());
    let key = hasher.finalize();

    Ok(key.to_vec())
}
