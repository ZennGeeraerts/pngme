mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::Parser;

use crate::args::{Args, Commands};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => commands::encode(file_path, chunk_type, message, output_file)?,
        Commands::Decode {
            file_path,
            chunk_type,
        } => commands::decode(file_path, chunk_type)?,
        Commands::Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type)?,
        Commands::Print { file_path } => commands::print(file_path)?,
        _ => return Err(Box::<dyn std::error::Error>::from("Unknown command")),
    };

    Ok(())
}
