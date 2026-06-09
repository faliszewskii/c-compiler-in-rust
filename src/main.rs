mod compiler;
mod translation_phases;
mod error;

extern crate clap;

use clap::Parser;
use env::consts::EXE_SUFFIX;
use log::trace;
use std::env;
use std::path::PathBuf;

use error::compiler_error::CompilerError;

const PROGRAM_NAME: &str = "C compiler in Rust";
const PROGRAM_DESC: &str = "A C language compiler written in Rust.";

#[derive(Parser, Debug)]
#[command(version, name = PROGRAM_NAME, about = PROGRAM_DESC)]
struct Args {
    /// Paths to input C source files to compile
    inputs: Vec<PathBuf>,

    /// Path to the output executable
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), CompilerError> {
    env_logger::init();

    let args = Args::parse();
    trace!("Parsed the following args: {:?}", args);

    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("app{}", EXE_SUFFIX)));

    compiler::compile(&args.inputs, output);

    Ok(())
}
