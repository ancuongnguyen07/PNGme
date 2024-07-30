mod cmd;
mod error;
mod png;

use crate::cmd::args::{App, Command};
use crate::cmd::commands::*;
use crate::error::Error;
use clap::Parser;

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
