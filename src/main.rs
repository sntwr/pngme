mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;
use commands::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(enc) => commands::encode(enc),
        Commands::Decode(dec) => commands::decode(dec),
        Commands::Remove(rem) => commands::remove(rem),
        Commands::Print(prn) => commands::print(prn),
    }
}
