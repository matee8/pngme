use crate::chunk_type::ChunkType;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(StructOpt)]
pub struct EncodeArgs {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
    #[structopt(parse(try_from_str = ChunkType::from_str))]
    pub chunk_type: ChunkType,
    pub message: String,
    #[structopt(parse(from_os_str))]
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt)]
pub struct DecodeArgs {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
    #[structopt(parse(try_from_str = ChunkType::from_str))]
    pub chunk_type: ChunkType,
}

#[derive(StructOpt)]
pub struct RemoveArgs {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
    #[structopt(parse(try_from_str = ChunkType::from_str))]
    pub chunk_type: ChunkType,
}

#[derive(StructOpt)]
pub struct PrintArgs {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
}

#[derive(StructOpt)]
pub enum Subcommand {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
