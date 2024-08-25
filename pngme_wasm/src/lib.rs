mod utils;

use pngme_core::img_format::TAG;

use std::str::FromStr;

use pngme_core::crypto;
use pngme_core::img_format::{Chunk, ChunkType, Png};
use wasm_bindgen::prelude::*;

use base64::{engine::general_purpose::STANDARD, Engine as _};

type Result<T> = std::result::Result<T, JsError>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PublicMaterial {
    encoded_bytes: Box<[u8]>,
    nonce: String,
}

#[wasm_bindgen]
impl PublicMaterial {
    #[wasm_bindgen(getter)]
    pub fn encoded_bytes(&self) -> Box<[u8]> {
        self.encoded_bytes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self.nonce.clone()
    }
}

#[wasm_bindgen]
pub fn encode(
    input_data: &[u8],
    passphrase: &str,
    message: &str,
    chunk_type: &str,
) -> Result<PublicMaterial> {
    log!("Encoding....");
    // let mut png = Png::try_from_file(Path::new(input_data)).map_err(JsError::from)?;
    let mut png = Png::try_from(input_data).map_err(JsError::from)?;
    log!("Tried_from raw bytes to png...done");

    let chunk_type = ChunkType::from_str(chunk_type).map_err(JsError::from)?;
    log!("Converting chunk_type...done");
    let key = crypto::sha3_hash(passphrase).map_err(JsError::from)?;
    let (chunk_content, nonce_raw) =
        crypto::aes256gcm_encrypt(message.as_bytes(), &key).map_err(JsError::from)?;
    log!("Encrypting...done");

    let new_chunk = Chunk::new(chunk_type, &chunk_content);
    png.append_chunk(new_chunk, true).map_err(JsError::from)?;
    log!("Appending chunk...done");

    // base64-encode the raw nonce
    let nonce = STANDARD.encode(nonce_raw);
    let encoded_bytes = png.as_bytes().into_boxed_slice();

    Ok(PublicMaterial {
        encoded_bytes,
        nonce,
    })
}

#[wasm_bindgen]
pub fn decode(
    input_data: &[u8],
    passphrase: &str,
    nonce: &str,
    chunk_type: &str,
) -> Result<String> {
    log!("Decoding...");
    let png = Png::try_from(input_data).map_err(JsError::from)?;
    log!("Tried_from raw bytes to png...done");

    let key = crypto::sha3_hash(passphrase).map_err(JsError::from)?;
    let nonce = STANDARD
        .decode(nonce)
        .map_err(|_| JsError::new("Invalid nonce"))?;
    log!("Base64-decoding Nonce...done");
    if let Some(mess_chunk) = png.chunk_by_type(chunk_type).map_err(JsError::from)? {
        let ciphertext = mess_chunk
            .data()
            .strip_prefix(&TAG)
            .ok_or(JsError::new("Tag missing"))?;
        log!("Found a hidden message");
        let plaintext_bytes =
            crypto::aes256gcm_decrypt(ciphertext, &key, &nonce).map_err(JsError::from)?;
        log!("Decrypting...done");
        Ok(String::from_utf8_lossy(&plaintext_bytes).to_string())
    } else {
        Err(JsError::new("Hidden message not found"))
    }
}
