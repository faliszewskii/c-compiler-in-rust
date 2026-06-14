use crate::compilation_stage::CompilationStage;
use crate::compiler_settings::CompilerSettings;
use crate::error::CompilerError;
use crate::error::CompilerError::ShortCircuit;
use crate::translation_phases::translation_phase_1::translation_phase_1;
use crate::translation_phases::translation_phase_2::translation_phase_2;
use crate::translation_phases::translation_phase_3::{preprocessing_tokens, translation_phase_3};
use preprocessing_tokens::PreprocessingTokens;
use std::path::{Path, PathBuf};
use CompilerError::UserError;

pub fn compile(
    input_files: &[PathBuf],
    output_file: &Path,
    settings: &CompilerSettings,
) -> Result<(), CompilerError> {
    let _preprocessing_result = input_files
        .iter()
        .map(|input_file: &PathBuf| preprocess_file(input_file, output_file, settings))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

fn preprocess_file(
    input_file: &Path,
    output_file: &Path,
    settings: &CompilerSettings,
) -> Result<PreprocessingTokens, CompilerError> {
    let output_dir = output_file
        .parent()
        .ok_or(UserError("Couldn't get outputs parent dir".to_string()))?;

    // Translation phase 1: line endings and trigraphs
    let bytes = read_bytes(input_file)?;
    let bytes = translation_phase_1(&bytes);
    save_step(&bytes, input_file, output_dir, "tp1")?;
    if settings.until == Some(CompilationStage::TranslationPhase1) {
        Err(ShortCircuit())?
    }

    // Translation phase 2: Line concatenation
    let bytes = translation_phase_2(&bytes);
    save_step(&bytes, input_file, output_dir, "tp2")?;
    if settings.until == Some(CompilationStage::TranslationPhase2) {
        Err(ShortCircuit())?
    }

    // Translation phase 3: Preprocessor tokens
    let source = String::from_utf8_lossy(&bytes).to_string();
    let preprocessing_tokens = translation_phase_3(&source)?;
    save_step(
        preprocessing_tokens.to_string().as_ref(),
        input_file,
        output_dir,
        "tp3",
    )?;
    if settings.until == Some(CompilationStage::TranslationPhase3) {
        Err(ShortCircuit())?
    }

    Ok(preprocessing_tokens)
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
