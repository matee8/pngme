use crate::{args, chunk::Chunk, png::Png, Result};

use std::{convert::TryFrom, fs, path::PathBuf};

pub fn encode(args: args::EncodeArgs) -> Result<()> {
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

pub fn decode(args: args::DecodeArgs) -> Result<()> {
    let input_bytes: Vec<u8> = fs::read(args.file_path)?;
    let img: Png = Png::try_from(input_bytes.as_slice())?;
    let chunk: Option<&Chunk> = img.chunk_by_type(&args.chunk_type.to_string());
    match chunk {
        Some(c) => println!("{}", c),
        None => println!("No chunks found"),
    }
    Ok(())
}

pub fn remove(args: args::RemoveArgs) -> crate::Result<()> {
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

pub fn print(args: args::PrintArgs) -> crate::Result<()> {
    let input_bytes: Vec<u8> = fs::read(args.file_path)?;
    let img: Png = Png::try_from(input_bytes.as_slice())?;
    for chunk in img.chunks().iter() {
        println!("{}", chunk);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_is_working() {
        crate::commands::print(crate::args::PrintArgs {
            file_path: "~/Downlodas/image.png".into(),
        });
    }
}
