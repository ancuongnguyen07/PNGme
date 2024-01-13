mod args;
mod chunk;
mod chunk_type;
mod commands;
mod displayable_vec;
mod error;
mod png;

use crate::error::Error;
use args::{App, Command};
use clap::Parser;
use commands::*;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = App::parse();

    match cli.command {
        Command::Decode(args) => decode(args),
        Command::Encode(args) => encode(args),
        Command::Remove(args) => remove(args),
        Command::Print(args) => print_chunks(args),
    }
}
