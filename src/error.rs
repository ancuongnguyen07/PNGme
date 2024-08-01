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

    #[error("Could not read from buffer")]
    BufferReaderErr(#[source] io::Error),

    #[error("Could not write to the buffer")]
    BufferWriterErr(#[source] io::Error),

    #[error("Please specify {0}")]
    MissingArg(String),

    #[error("Could not find the secret message")]
    NotFoundSecMess,

    #[error("Could not find the chunk type")]
    NotFoundChunkType,

    #[error("Could not read file: {0}")]
    FileOpenErr(#[source] io::Error),

    #[error("Could not write file: {0}")]
    FileWriteErr(#[source] io::Error),

    #[error("Could not download file from the given URL: {0}")]
    CurlErr(String),

    #[error("The URL should link to a PNG file")]
    InvalidPNGURL(String),
}
