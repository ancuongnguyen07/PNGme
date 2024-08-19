use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    CoreLibErr(#[source] pngme_core::Error),

    #[error("Could not download file from the given URL: {0}")]
    CurlErr(String),

    #[error("Please specify {0}")]
    MissingArg(String),

    #[error("Could not write file: {0}")]
    FileWriteErr(#[source] io::Error),

    #[error("Invalid Nonce: {0}")]
    InvalidNonce(String),

    #[error("Could not find any tagged message")]
    TagMissing,

    #[error("Could not read your passphrase")]
    PassphraseReadErr,

    #[error("You have to choose one of two options: -k OR -p")]
    OverlapKeyPassphrase,

    #[error("Could not decode your base64-encoded key")]
    InvalidKey,

    #[error("Could find your secret message")]
    NotFoundSecMess,
}
