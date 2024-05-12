use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs, Subcommand},
    chunk::Chunk,
    png::Png,
    Result,
};

use std::{convert::TryFrom, fs, path::PathBuf};

pub fn run(subcmd: Subcommand) -> Result<()> {
    match subcmd {
        Subcommand::Encode(args) => encode(args),
        Subcommand::Decode(args) => decode(args),
        Subcommand::Remove(args) => remove(args),
        Subcommand::Print(args) => print(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let input_bytes: Vec<u8> = fs::read(&args.file_path)?;
    let output: PathBuf = match args.output_file {
        Some(val) => val,
        None => args.file_path,
    };
    let mut img: Png = Png::try_from(input_bytes.as_slice())?;
    let chunk: Chunk =
        Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    img.append_chunk(chunk);
    fs::write(output, img.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let input_bytes: Vec<u8> = fs::read(args.file_path)?;
    let img: Png = Png::try_from(input_bytes.as_slice())?;
    let chunk: Option<&Chunk> = img.chunk_by_type(&args.chunk_type.to_string());
    match chunk {
        Some(c) => println!("{}", c),
        None => println!("No chunks found"),
    }
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let input_bytes = fs::read(&args.file_path)?;
    let mut img = Png::try_from(input_bytes.as_slice())?;
    match img.remove_chunk(&args.chunk_type.to_string()) {
        Ok(chunk) => {
            fs::write(&args.file_path, img.as_bytes())?;
            println!("Removed chunk: {}", chunk);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let input_bytes: Vec<u8> = fs::read(args.file_path)?;
    let img: Png = Png::try_from(input_bytes.as_slice())?;
    for chunk in img.chunks().iter() {
        println!("{}", chunk);
    }
    Ok(())
}
