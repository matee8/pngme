use args::Cli;
use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli: Cli = Cli::from_args();
    commands::run(cli.subcmd)
}
