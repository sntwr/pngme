use std::error::Error;
use std::str::FromStr;
use std::fs;

use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::png::{Png,PngError};

use crate::args::{EncodeArgs,DecodeArgs,RemoveArgs,PrintArgs};

pub fn encode(args: &EncodeArgs) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.input_file_path)?;
    let mut png = Png::try_from(data.as_ref())?;

    let end_chunk = png.remove_chunk("IEND")?;
    let chunk_type = ChunkType::from_str(&args.chunk_type_str)?;
    let new_chunk = Chunk::new(chunk_type, args.message.clone().into_bytes());
    png.append_chunk(new_chunk);
    png.append_chunk(end_chunk);

    if args.output_file_path.is_some() {
        fs::write(args.output_file_path.as_ref().unwrap(), png.as_bytes())?;
    } else {
        fs::write(&args.input_file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.input_file_path)?;
    let png = Png::try_from(data.as_ref())?;

    let chunk = png.chunk_by_type(&args.chunk_type_str).ok_or(PngError::ChunkNotFound)?;
    let chunk_data = chunk.data_as_string()?;
    println!("{}", chunk_data);
    Ok(())
}

pub fn remove(args: &RemoveArgs) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.input_file_path)?;
    let mut png = Png::try_from(data.as_ref())?;

    png.remove_chunk(&args.chunk_type_str)?;
    fs::write(&args.input_file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: &PrintArgs) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.input_file_path)?;
    let png = Png::try_from(data.as_ref())?;

    println!("{}", png);
    Ok(())
}