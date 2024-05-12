use crate::chunk_type::ChunkType;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(StructOpt)]
pub struct EncodeArgs {
    #[structopt(parse(from_os_str), help = "Input file path")]
    pub file_path: PathBuf,
    #[structopt(
        parse(try_from_str = ChunkType::from_str), 
        help = "Chunk type (like \"TeSt\")"
    )]
    pub chunk_type: ChunkType,
    #[structopt(help = "Hidden message.")]
    pub message: String,
    #[structopt(parse(from_os_str), help = "Output file path (optional)")]
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt)]
pub struct DecodeArgs {
    #[structopt(parse(from_os_str), help = "Path to the .png file")]
    pub file_path: PathBuf,
    #[structopt(
        parse(try_from_str = ChunkType::from_str),
        help = "Chunk type (like \"TeSt\")"
    )]
    pub chunk_type: ChunkType,
}

#[derive(StructOpt)]
pub struct RemoveArgs {
    #[structopt(parse(from_os_str), help = "Path to the .png file.")]
    pub file_path: PathBuf,
    #[structopt(
        parse(try_from_str = ChunkType::from_str), 
        help = "Chunk type (like \"TeSt\")"
    )]
    pub chunk_type: ChunkType,
}

#[derive(StructOpt)]
pub struct PrintArgs {
    #[structopt(parse(from_os_str), help = "Path to the .png file")]
    pub file_path: PathBuf,
}

#[derive(StructOpt)]
pub enum Subcommand {
    #[structopt(about = "Hide a secret message in a .png file")]
    Encode(EncodeArgs),
    #[structopt(about = "Show the secret message in a .png file")]
    Decode(DecodeArgs),
    #[structopt(about = "Remove a secret message from a .png file")]
    Remove(RemoveArgs),
    #[structopt(about = "Print the whole .png file")]
    Print(PrintArgs),
}
