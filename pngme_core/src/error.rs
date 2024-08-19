use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid legnth of the input byte: {0}")]
    InvalidLength(String),

    #[error("Invalid byte value, only [a-zA-Z]")]
    InvalidByteValue,

    #[error("Invalid header byte: Only PNG header byte is allowed")]
    InvalidHeaderByte,

    #[error("Could not convert a vector of bytes to string")]
    StringConversion,

    #[error("Invalid CRC")]
    InvalidCRC,

    #[error("The Chunk Type is already existed")]
    DuplicatedChunkType,

    #[error("Could not read from buffer")]
    BufferReaderErr(#[source] io::Error),

    #[error("Could not write to the buffer")]
    BufferWriterErr(#[source] io::Error),

    #[error("Could not find the chunk type")]
    NotFoundChunkType,

    #[error("Could not read file: {0}")]
    FileOpenErr(#[source] io::Error),

    #[error("The URL should link to a PNG file")]
    InvalidPNGURL(String),

    #[error("Invalid key length for AES256: the key should be 32-byte long")]
    InvalidKeyLength,

    #[error("Invalid key for AES256")]
    InvalidKey,

    #[error("Could not encrypt your data chunk")]
    EncryptionErr,

    #[error("Could not decrypt your secret message, maybe you used a wrong passphrase or base64-encoded key")]
    DecryptionErr,
}
