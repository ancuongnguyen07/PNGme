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
    pub in_file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,

    /// Secret message
    #[arg(short, long)]
    pub mess: String,

    /// Path to the output PNG file
    #[arg(short, long)]
    pub out_file_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to the input PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to the input PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,
}
