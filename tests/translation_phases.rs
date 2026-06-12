use std::env::current_dir;
use std::path::PathBuf;

mod test_utils;

#[test]
fn test_translation_phase_1() {
    let inputs = [PathBuf::from(current_dir().unwrap()).join("tests/resources/input/tp1.c")];
    let expecteds = [PathBuf::from(current_dir().unwrap()).join("tests/resources/expected/tp1.tp1")];

    let temp = tempfile::tempdir().unwrap();
    test_utils::test_compiler_output(inputs.into(), temp.path().join("app").as_ref(), expecteds.into());
}
