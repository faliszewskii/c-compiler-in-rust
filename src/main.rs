mod compiler;
mod error;
mod translation_phases;

extern crate clap;

use clap::Parser;
use const_format::concatcp;
use env::consts::EXE_SUFFIX;
use error::compiler_error::CompilerError;
use log::trace;
use std::env;
use std::path::PathBuf;

const PROGRAM_NAME: &str = "C compiler in Rust";
const PROGRAM_DESC: &str = "A C language compiler written in Rust.";
const DEFAULT_EXE: &str = concatcp!("app", EXE_SUFFIX);

#[derive(Parser, Debug)]
#[command(version, name = PROGRAM_NAME, about = PROGRAM_DESC)]
struct Args {
    /// Paths to input C source files to compile
    inputs: Vec<PathBuf>,

    /// Path to the output executable
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn handle_compiler_error(err: CompilerError) -> anyhow::Result<()> {
    match err {
        CompilerError::UserError(_) => err.report_and_exit(),
        CompilerError::Internal(internal_error) => Err(internal_error),
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    trace!("Parsed the following args: {:?}", args);

    let output = args.output.unwrap_or_else(|| PathBuf::from(DEFAULT_EXE));

    let result = compiler::compile(&args.inputs, output);

    if let Err(err) = result {
        return handle_compiler_error(err);
    }
    Ok(())
}
