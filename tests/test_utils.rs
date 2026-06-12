use assert_cmd::Command;
use std::path::{Path, PathBuf};

pub fn test_compiler_output(inputs: Vec<PathBuf>, output: &Path, expecteds: Vec<PathBuf>) {
    let mut bin = Command::cargo_bin("c-compiler-in-rust").unwrap();
    for input in &inputs {
        assert!(input.exists());
        bin.arg(input);
    }
    bin.arg("-o").arg(output);
    bin.assert().success();

    let output_dir = output.parent().unwrap().to_path_buf();
    for expected in &expecteds {
        assert!(expected.exists());
        let file_name = expected.file_name().unwrap();
        let output = output_dir.join(file_name);
        assert!(output.exists());
        assert_eq!(
            std::fs::read(expected).unwrap(),
            std::fs::read(output).unwrap()
        );
    }
}
