mod compilation_stage;
mod compiler;
mod compiler_settings;
mod error;
mod lexical_analysis;
mod translation_phases;

extern crate clap;

use crate::compilation_stage::CompilationStage;
use crate::compiler_settings::CompilerSettings;
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

    /// Stage at which to stop the compilation
    #[arg(long)]
    until: Option<CompilationStage>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    trace!("Parsed the following args: {:?}", args);

    let output = args.output.unwrap_or_else(|| PathBuf::from(DEFAULT_EXE));

    let settings = CompilerSettings { until: args.until };

    let result = compiler::compile(&args.inputs, &output, &settings);

    if let Err(err) = result {
        handle_compiler_error(err)?
    }
    Ok(())
}

fn handle_compiler_error(err: CompilerError) -> anyhow::Result<()> {
    match err {
        CompilerError::ShortCircuit() => Ok(()),
        CompilerError::UserError(_) => err.report_and_exit(),
        CompilerError::Internal(internal_error) => Err(internal_error),
    }
}
