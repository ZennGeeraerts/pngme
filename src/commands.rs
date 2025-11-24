use crate::chunk::{self, Chunk};
use crate::chunk_type::{self, ChunkType};
use crate::png::Png;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub fn encode(
    file_path: &PathBuf,
    chunk_type_str: &str,
    message: &str,
    output_file: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let byte_array = fs::read(file_path)?;
    let mut png = Png::try_from(byte_array.as_slice())?;

    let chunk_type = ChunkType::from_str(chunk_type_str)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);

    if let Some(file) = output_file {
        fs::write(file, png.as_bytes())?;
    }

    return Ok(());
}

pub fn decode(file_path: &PathBuf, chunk_type_str: &str) -> Result<(), Box<dyn Error>> {
    let byte_array = fs::read(file_path)?;
    let png = Png::try_from(byte_array.as_slice())?;

    match png.chunk_by_type(chunk_type_str) {
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
            Ok(())
        }
        None => Err(format!("No chunk of type '{}' found", chunk_type_str).into()),
    }
}

pub fn remove(file_path: &PathBuf, chunk_type_str: &str) -> Result<(), Box<dyn Error>> {
    let byte_array = fs::read(file_path)?;
    let mut png = Png::try_from(byte_array.as_slice())?;

    png.remove_first_chunk(chunk_type_str)
        .map_err(|e| format!("Error removing chunk: {}", e))?;

    fs::write(file_path, png.as_bytes())?;

    return Ok(());
}

pub fn print(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let byte_array = fs::read(file_path)?;
    let png = Png::try_from(byte_array.as_slice())?;
    println!("{}", png);
    return Ok(());
}
