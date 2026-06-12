use crate::error::CompilerError;
use crate::translation_phases::translation_phase_1::translation_phase_1;
use std::path::PathBuf;
use CompilerError::UserError;

fn read_bytes(f: &PathBuf) -> Result<Vec<u8>, CompilerError> {
    std::fs::read(f).map_err(|_| UserError(format!("Could not read from file {}", f.display())))
}

fn save_bytes(f: &PathBuf, bytes: &[u8]) -> Result<(), CompilerError> {
    std::fs::write(f, bytes)
        .map_err(|_| UserError(format!("Could not write to file {}", f.display())))
}

pub fn compile(input_files: &[PathBuf], output_file: PathBuf) -> Result<(), CompilerError> {
    // translation phase 1: line endings and trigraphs.
    let phase_1_result: Vec<Vec<u8>> = input_files
        .iter()
        .map(read_bytes)
        .map(|r| r.and_then(|bytes| Ok(translation_phase_1(&bytes))))
        .collect::<Result<_, _>>()?;

    // Save tp1 results
    let output_dir = output_file
        .parent()
        .ok_or(UserError("Couldn't get outputs parent dir".to_string()))?;
    phase_1_result
        .iter()
        .zip(input_files.iter())
        .try_for_each(|(phase1, input_file)| {
            let input_file_name = input_file.file_name().ok_or(UserError(format!(
                "Could get file name for {}",
                input_file.display()
            )))?;
            let file_name = output_dir.join(input_file_name).with_extension("tp1");
            save_bytes(&file_name, phase1)
        })?;

    Ok(())
}
