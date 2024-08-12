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
    /// Encodes a message into a PNG file and save the result
    Encode(EncodeArgs),
    /// Searches for a message hidden in a PNG file and prints the message if one is found
    Decode(DecodeArgs),
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

    /// URL to a PNG image
    #[arg(short, long)]
    pub url: Option<String>,

    /// The base64-encoded private key for encryption
    #[arg(short, long)]
    pub key: Option<String>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the PNG file
    #[arg(short, long, required = true)]
    pub in_file_path: PathBuf,

    /// Chunk type: 4 bytes
    #[arg(short, long, required = true)]
    pub chunk_type: String,

    /// The base64-encoded private key for decryption
    #[arg(short, long)]
    pub key: Option<String>,

    /// The Nonce used for decryption
    #[arg(short, long)]
    pub nonce: Option<String>,
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
