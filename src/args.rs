use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pngme", about = "")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(help = "File path of the png file to encode")]
        file_path: PathBuf,

        #[arg(help = "Chunk type of the png file to encode")]
        chunk_type: String,

        #[arg(help = "The message to encode")]
        message: String,

        #[arg(help = "The optional output file")]
        output_file: Option<PathBuf>,
    },
    Decode {
        #[arg(help = "File path of the png file to decode")]
        file_path: PathBuf,

        #[arg(help = "Chunk type of the png file to decode")]
        chunk_type: String,
    },
    Remove {
        #[arg(help = "File path of the png file to remove")]
        file_path: PathBuf,

        #[arg(help = "Chunk type of the png file to remove")]
        chunk_type: String,
    },
    Print {
        #[arg(help = "File path of the png file to print")]
        file_path: PathBuf,
    },
}
