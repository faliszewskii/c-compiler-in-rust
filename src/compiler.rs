use crate::error::CompilerError;
use crate::translation_phases::translation_phase_1::translation_phase_1;
use crate::translation_phases::translation_phase_2::translation_phase_2;
use CompilerError::UserError;
use std::path::{Path, PathBuf};

pub fn compile(input_files: &[PathBuf], output_file: &Path) -> Result<(), CompilerError> {
    let preprocessing_result: Vec<Vec<u8>> = input_files
        .iter()
        .map(|input_file: &PathBuf| preprocess_file(input_file, output_file))
        .collect::<Result<_, _>>()?;
    Ok(())
}

fn preprocess_file(input_file: &Path, output_file: &Path) -> Result<Vec<u8>, CompilerError> {
    let output_dir = output_file
        .parent()
        .ok_or(UserError("Couldn't get outputs parent dir".to_string()))?;

    let bytes = read_bytes(input_file)?;
    let bytes = translation_phase_1(&bytes);
    save_step(&bytes, input_file, output_dir, "tp1")?;
    let bytes = translation_phase_2(&bytes);
    save_step(&bytes, input_file, output_dir, "tp2")?;
    Ok(bytes)
}

fn save_step(
    bytes: &[u8],
    input_file: &Path,
    output_dir: &Path,
    extension: &str,
) -> Result<(), CompilerError> {
    let input_file_name = input_file.file_name().ok_or(UserError(format!(
        "Could get file name for {}",
        input_file.display()
    )))?;
    let file_name = output_dir.join(input_file_name).with_extension(extension);
    save_bytes(&file_name, bytes)
}

fn read_bytes(f: &Path) -> Result<Vec<u8>, CompilerError> {
    std::fs::read(f).map_err(|_| UserError(format!("Could not read from file {}", f.display())))
}

fn save_bytes(f: &Path, bytes: &[u8]) -> Result<(), CompilerError> {
    std::fs::write(f, bytes)
        .map_err(|_| UserError(format!("Could not write to file {}", f.display())))
}
