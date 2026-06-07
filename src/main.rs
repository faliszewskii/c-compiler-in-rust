mod compiler;

extern crate clap;

use env::consts::EXE_SUFFIX;
use std::env;
use std::error::Error;
use clap::Parser;
use std::path::{PathBuf};
use log::trace;

const PROGRAM_NAME: &str = "C compiler in Rust";
const PROGRAM_DESC: &str = "A C language compiler written in Rust.";

#[derive(Parser, Debug)]
#[command(version, name = PROGRAM_NAME, about = PROGRAM_DESC)]
struct Args {
    /// Path to input C source files to compile
    inputs: Vec<PathBuf>,

    /// Path to the output executable
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    trace!("Parsed the following args: {:?}", args);

    let output = args.output.unwrap_or_else(|| {
        PathBuf::from(format!("app{}", EXE_SUFFIX))
    });

    compiler::compile(args.inputs, output);

    Ok(())
}
