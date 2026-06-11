use crate::error::CompilerError;
use crate::translation_phases::translation_phase_1::translation_phase_1;
use CompilerError::UserError;
use std::path::PathBuf;

fn read_bytes(f: &PathBuf) -> Result<Vec<u8>, CompilerError> {
    std::fs::read(f).map_err(|_| UserError(format!("Could not read from file {}", f.display())))
}

fn save_bytes(f: &PathBuf, bytes: &[u8]) -> Result<(), CompilerError> {
    std::fs::write(f, bytes)
        .map_err(|_| UserError(format!("Could not write to file {}", f.display())))
}

pub fn compile(input_files: &[PathBuf], output_file: PathBuf) -> Result<(), CompilerError> {        
    let phase_1_result: Vec<Vec<u8>> = input_files
        .iter()
        .map(read_bytes)
        .map(|r| r.and_then(|bytes| Ok(translation_phase_1(&bytes))))
        .collect::<Result<_, _>>()?;
    phase_1_result
        .iter()
        .zip(input_files.iter())
        .try_for_each(|(phase1, input_file)| {
            save_bytes(&input_file.with_extension("tp1"), phase1)
        })?;
    Ok(())
}
