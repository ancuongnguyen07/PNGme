use crate::error::Error;
use pngme_core::crypto::{aes256gcm_decrypt, aes256gcm_encrypt, sha3_hash};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use curl::easy::Easy;

use crate::cmd::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs, SearchArgs};
use crate::Result;
use pngme_core::img_format::Chunk;
use pngme_core::img_format::ChunkType;
use pngme_core::img_format::Png;

use std::fs::File;
use std::io::Write;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use pngme_core::img_format::TAG;

/// Helper function for the `Encode` command.
/// Returns base64-encoded ciphertext and Nonce.
fn encrypt_helper(
    key_arg: &Option<String>,
    passphrase_arg: &Option<String>,
    mess_bytes: &[u8],
) -> Result<(Vec<u8>, String)> {
    if passphrase_arg.is_some() && key_arg.is_some() {
        return Err(Error::OverlapKeyPassphrase);
    }

    let enc_key = if let Some(base64_enc_key) = key_arg {
        // Use the given key to encrypt message (chunk content)

        // Base64 decoding the encryption key
        STANDARD
            .decode(base64_enc_key)
            .map_err(|_| Error::InvalidKey)?
    } else {
        // Use the key derived from the given passphrase for encryption
        if let Some(passphrase) = passphrase_arg {
            // Hash the given passphrase
            sha3_hash(passphrase).map_err(Error::CoreLibErr)?
        } else {
            // Promt to user for typing their passphrase invisibly
            get_passphrase_key()?
        }
    };

    let (ciphertext, nonce_raw) =
        aes256gcm_encrypt(mess_bytes, &enc_key).map_err(Error::CoreLibErr)?;
    // base64-encode Nonce
    let nonce = STANDARD.encode(nonce_raw);
    Ok((ciphertext, nonce))
}

/// Helper function for the `Encode` command.
/// Parse `input_file_path` and `url` args and return the path to input PNG file.
fn input_png_helper(
    file_arg: &Option<PathBuf>,
    url_arg: &Option<String>,
    verbosity: bool,
) -> Result<PathBuf> {
    let png_file_path = if let Some(file_path) = file_arg {
        // PNG file is loaded at rest
        Ok(file_path.clone())
    } else {
        // PNG file is downloaded via curl
        if let Some(url) = url_arg {
            let mut easy_curl = Easy::new();
            easy_curl
                .url(url)
                .map_err(|_| Error::CurlErr(url.clone()))?;

            let file_name = url.split('/').last().ok_or(Error::CurlErr(url.clone()))?;
            let mut download_png = File::create(file_name).map_err(Error::FileWriteErr)?;
            easy_curl
                .write_function(move |data| {
                    download_png.write_all(data).unwrap();
                    Ok(data.len())
                })
                .map_err(|_| Error::CurlErr(url.clone()))?;
            easy_curl
                .perform()
                .map_err(|_| Error::CurlErr(url.clone()))?;

            if verbosity {
                println!("Downloading {url}... done")
            }
            Ok(PathBuf::from_str(file_name).unwrap())
        } else {
            Err(Error::MissingArg("Input PNG file or URL".to_string()))
        }
    }?;

    Ok(png_file_path)
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let file_path = input_png_helper(&args.in_file_path, &args.url, args.verbosity)?;
    let mut png = Png::try_from(Path::new(&file_path)).map_err(Error::CoreLibErr)?;
    if args.verbosity {
        println!("Reading {}... done", file_path.to_string_lossy());
    }

    let chunk_type = ChunkType::from_str(&args.chunk_type).map_err(Error::CoreLibErr)?;
    let (chunk_content, nonce) = encrypt_helper(&args.key, &args.passphrase, args.mess.as_bytes())?;
    let chunk = Chunk::new(chunk_type, &chunk_content);
    if args.verbosity {
        println!("Encrypting your message... done");
    }

    png.append_chunk(chunk, true).map_err(Error::CoreLibErr)?;

    png.to_file(Path::new(&args.out_file_path))
        .map_err(Error::CoreLibErr)?;
    if args.verbosity {
        println!(
            "Embeding your secret message to {}... done",
            args.out_file_path.to_string_lossy()
        );
        println!("Please SAVE a copy of this base64-encoded Nonce so that you can use it to decrypt your message later: {nonce}");
    } else {
        println!("Nonce:{nonce}");
    }

    Ok(())
}

fn decrypt_helper(
    ciphertext: &[u8],
    passphrase_arg: &Option<String>,
    key_arg: &Option<String>,
    nonce: &str,
) -> Result<Vec<u8>> {
    if passphrase_arg.is_some() && key_arg.is_some() {
        return Err(Error::OverlapKeyPassphrase);
    }

    let dec_key = if let Some(base64_dec_key) = key_arg {
        STANDARD
            .decode(base64_dec_key)
            .map_err(|_| Error::InvalidKey)?
    } else if let Some(passphrase) = passphrase_arg {
        sha3_hash(passphrase).map_err(Error::CoreLibErr)?
    } else {
        get_passphrase_key()?
    };

    let nonce = STANDARD
        .decode(nonce)
        .map_err(|_| Error::InvalidNonce(nonce.to_string()))?;
    aes256gcm_decrypt(ciphertext, &dec_key, &nonce).map_err(Error::CoreLibErr)
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::try_from(Path::new(&args.in_file_path)).map_err(Error::CoreLibErr)?;
    if args.verbosity {
        println!("Reading {}... done", args.in_file_path.to_string_lossy());
    }

    if let Some(mess_chunk) = png
        .chunk_by_type(&args.chunk_type)
        .map_err(Error::CoreLibErr)?
    {
        let ciphertext = mess_chunk
            .data()
            .strip_prefix(&TAG)
            .ok_or(Error::TagMissing)?;
        let mess_bytes = decrypt_helper(ciphertext, &args.passphrase, &args.key, &args.nonce)?;
        if args.verbosity {
            println!("Decrypting your secret message... done");
        }

        let mess = String::from_utf8_lossy(&mess_bytes);
        if args.verbosity {
            println!("Your secret message: {mess}");
        } else {
            println!("Message:{mess}");
        }
        Ok(())
    } else {
        Err(Error::NotFoundSecMess)
    }
}

fn search_helper(png: &Png) -> Vec<&Chunk> {
    png.chunks()
        .iter()
        .filter(|&chunk| chunk.data().starts_with(&TAG))
        .collect()
}

pub fn search(args: SearchArgs) -> Result<()> {
    let png = Png::try_from(Path::new(&args.in_file_path)).map_err(Error::CoreLibErr)?;
    let candidates = search_helper(&png);
    for (ind, chunk) in candidates.iter().enumerate() {
        let mess_str = String::from_utf8_lossy(chunk.data());
        println!("Message {}: {}", ind + 1, mess_str);
    }

    let total = candidates.len();
    if args.verbosity {
        println!("PNGme has found {total} potential hidden messages");
    } else {
        println!("Total:{total}");
    }
    Ok(())
}

/// Catches the passphrase typed by a user, then
/// SHA3-hasing it to derive a symmetric key for encryption/decryption
fn get_passphrase_key() -> Result<Vec<u8>> {
    let passphrase = rpassword::prompt_password("Enter your passphrase: ")
        .map_err(|_| Error::PassphraseReadErr)?;
    sha3_hash(&passphrase).map_err(Error::CoreLibErr)
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::try_from(Path::new(&args.in_file_path)).map_err(Error::CoreLibErr)?;
    match png.remove_chunk(&args.chunk_type) {
        Ok(_) => {
            png.to_file(Path::new(&args.in_file_path))
                .map_err(Error::CoreLibErr)?;
            Ok(())
        }
        Err(err) => Err(Error::CoreLibErr(err)),
    }
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::try_from(Path::new(&args.in_file_path)).map_err(Error::CoreLibErr)?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "5f6/dVmvW1c/lxQ/22Mqax/RvhzzZ4a5EBFCXYt3K4w=";
    const MESSAGE: &str = "FooBar!";

    fn encrypt_decrypt_helper(
        message: &str,
        key_arg: &Option<String>,
        passphrase_arg: &Option<String>,
    ) -> Result<()> {
        let (ciphertext, nonce) = encrypt_helper(key_arg, passphrase_arg, message.as_bytes())?;
        let plaintext = decrypt_helper(ciphertext.as_slice(), passphrase_arg, key_arg, &nonce)?;
        let plaintext = String::from_utf8_lossy(&plaintext);
        assert_eq!(plaintext, message);
        Ok(())
    }

    #[test]
    fn test_aes_crypto_with_key() -> Result<()> {
        let key_arg = Some(KEY.to_string());
        let passphrase_arg: Option<String> = None;
        encrypt_decrypt_helper(MESSAGE, &key_arg, &passphrase_arg)?;
        Ok(())
    }

    #[test]
    fn test_aes_crypto_with_passphrase() -> Result<()> {
        let key_arg = None;
        let passphrase_arg = Some("HelloWorld!".to_string());
        encrypt_decrypt_helper(MESSAGE, &key_arg, &passphrase_arg)?;
        Ok(())
    }

    #[test]
    fn test_aes_crypto_with_passphrase_and_key() {
        let key_arg = Some(KEY.to_string());
        let passphrase_arg = Some("HelloWorld!".to_string());
        let result = encrypt_decrypt_helper(MESSAGE, &key_arg, &passphrase_arg);
        assert!(matches!(result, Err(Error::OverlapKeyPassphrase)));
    }
}
