mod cmd;
mod error;

use crate::cmd::args::{App, Command};
use crate::cmd::commands::*;
use crate::error::Error;
use clap::Parser;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = App::parse();

    let result = match cli.command {
        Command::Decode(args) => decode(args),
        Command::Encode(args) => encode(args),
        Command::Search(args) => search(args),
        Command::Remove(args) => remove(args),
        Command::Print(args) => print_chunks(args),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }
    Ok(())
}
