pub mod crypto;
mod error;
pub mod png;

pub use crate::error::Error;
type Result<T> = std::result::Result<T, Error>;
