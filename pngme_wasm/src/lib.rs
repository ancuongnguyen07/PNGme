mod utils;

use std::path::Path;
use std::str::FromStr;

use pngme_core::crypto;
use pngme_core::png::{Chunk, ChunkType, Png};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PublicMaterial {
    ciphertext: Box<[u8]>,
    nonce: String,
}

#[wasm_bindgen]
impl PublicMaterial {
    pub fn new(ciphertext: &[u8], nonce: &str) -> Self {
        PublicMaterial {
            ciphertext: ciphertext.to_vec().into_boxed_slice(),
            nonce: nonce.to_string(),
        }
    }
}

#[wasm_bindgen]
pub fn encrypt(
    file_path: &str,
    passphrase: &str,
    message: &str,
    chunk_type: &str,
) -> Result<PublicMaterial, JsError> {
    todo!();
    let png = Png::try_from_file(Path::new(file_path)).map_err(|e| JsError::from(e))?;
    let chunk_type = ChunkType::from_str(chunk_type).map_err(|e| JsError::from(e))?;
    let key = crypto::sha3_hash(passphrase).map_err(|e| JsError::from(e))?;
    let (chunk_content, nonce) =
        crypto::aes256gcm_encrypt(message.as_bytes(), &key).map_err(|e| JsError::from(e))?;

    let new_chunk = Chunk::new(chunk_type, &chunk_content);
    png.append_chunk(new_chunk, true);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello WASM!");
}

/// Capture filename from a filepath
fn get_filename() {
    todo!();
}
