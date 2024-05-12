use args::Subcommand;
use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Args::from_args();
    match cli.subcmd {
        Subcommand::Encode(args) => commands::encode(args),
        Subcommand::Decode(args) => commands::decode(args),
        Subcommand::Remove(args) => commands::remove(args),
        Subcommand::Print(args) => commands::print(args),
    }
}
