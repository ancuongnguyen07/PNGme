mod args;
mod chunk;
mod chunk_type;
mod commands;
mod displayable_vec;
mod png;

use args::{App, Command};
use clap::Parser;
use commands::*;

pub type Error = Box<dyn std::error::Error>;
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
