use std::error::Error;
use std::str::FromStr;
use std::fs;

use clap::{Args, Parser, Subcommand};

use crate::chunk_type::{ChunkType, ChunkTypeError};
use crate::chunk::{Chunk, ChunkError};
use crate::png::{Png,PngError};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Args, Debug)]
pub struct EncodeArgs {
    input_file_path: String,
    chunk_type_str: String,
    message: String,
    output_file_path: Option<String>,
}
#[derive(Args, Debug)]

pub struct DecodeArgs {
    input_file_path: String,
    chunk_type_str: String,
}
#[derive(Args, Debug)]

pub struct RemoveArgs {
    input_file_path: String,
    chunk_type_str: String,
}
#[derive(Args, Debug)]

pub struct PrintArgs {
    input_file_path: String,
}

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