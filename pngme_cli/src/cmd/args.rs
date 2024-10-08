use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// A magic tool to embed your message into a PNG image.
#[derive(Debug, Parser)]
#[clap(name = "PNGme", version = "0.1.0", author = "Cuong Nguyen")]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encodes a message into a PNG file
    Encode(EncodeArgs),
    /// Decode a hidden message a PNG file
    Decode(DecodeArgs),
    /// Search for potential hidden message
    Search(SearchArgs),
    /// Removes a chunk from a PNG file and saves the result
    Remove(RemoveArgs),
    /// Prints all of the chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// Path to the input PNG file
    #[arg(short, long)]
    pub in_file_path: Option<PathBuf>,

    /// Chunk type: 4 bytes
    #[arg(short, long, required = true)]
    pub chunk_type: String,

    /// Secret message
    #[arg(short, long, required = true)]
    pub mess: String,

    /// Path to the output PNG file
    #[arg(short, long, required = true)]
    pub out_file_path: PathBuf,

    /// Verbosity
    #[arg(short, long)]
    pub verbosity: bool,

    /// Exhaustively searching for all potential secret hidden message
    #[arg(short, long)]
    pub all: bool,

    /// URL to a PNG image
    #[arg(short, long)]
    pub url: Option<String>,

    /// The base64-encoded private key for encryption
    #[arg(short, long)]
    pub key: Option<String>,

    /// Passphrase used for encryption/decryption
    #[arg(short, long)]
    pub passphrase: Option<String>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the PNG file
    #[arg(short, long, required = true)]
    pub in_file_path: PathBuf,

    /// Chunk type: exactly 4 bytes
    #[arg(short, long, required = true)]
    pub chunk_type: String,

    /// The base64-encoded private key for decryption
    #[arg(short, long)]
    pub key: Option<String>,

    /// Verbosity
    #[arg(short, long)]
    pub verbosity: bool,

    /// The Nonce used for decryption
    #[arg(short, long, required = true)]
    pub nonce: String,

    /// Passphrase used for encryption/decryption
    #[arg(short, long)]
    pub passphrase: Option<String>,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Path to the input PNG file
    #[arg(short, long, required = true)]
    pub in_file_path: PathBuf,

    /// Verbosity
    #[arg(short, long)]
    pub verbosity: bool,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to the input PNG file
    #[arg(short, long, required = true)]
    pub in_file_path: PathBuf,

    /// Chunk type: 4 bytes
    #[arg(short, long, required = true)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to the input PNG file
    #[arg(short, long, required = true)]
    pub in_file_path: PathBuf,
}
