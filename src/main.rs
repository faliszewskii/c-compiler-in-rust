extern crate clap;

use clap::Parser;
use std::path::PathBuf;
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

fn main() {
    env_logger::init();
    let args = Args::parse();
    trace!("Parsed the following args: {:?}", args);
}
