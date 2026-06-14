use std::env::current_dir;
use assert_cmd::Command;

mod test_utils;

#[test]
fn test_translation_phase_1() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp1.c")];
    let expecteds = [current_dir().unwrap().join("tests/resources/expected/tp1/tp1.tp1")];

    let temp = tempfile::tempdir().unwrap();
    let mut bin = Command::cargo_bin("c-compiler-in-rust").unwrap();
    bin.arg("--until").arg("tp1");
    test_utils::test_compiler_output(&mut bin, &inputs, &temp.path().join("app"), &expecteds);
}

#[test]
fn test_translation_phase_2() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp2.c")];
    let expecteds = [
        current_dir().unwrap().join("tests/resources/expected/tp2/tp2.tp1"),
        current_dir().unwrap().join("tests/resources/expected/tp2/tp2.tp2"),
    ];

    let temp = tempfile::tempdir().unwrap();
    let mut bin = Command::cargo_bin("c-compiler-in-rust").unwrap();
    bin.arg("--until").arg("tp2");
    test_utils::test_compiler_output(&mut bin, &inputs, &temp.path().join("app"), &expecteds);
}

#[test]
fn test_translation_phase_3() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp3.c")];
    let expecteds = [
        current_dir().unwrap().join("tests/resources/expected/tp3/tp3.tp1"),
        current_dir().unwrap().join("tests/resources/expected/tp3/tp3.tp2"),
        current_dir().unwrap().join("tests/resources/expected/tp3/tp3.tp3"),
    ];

    let temp = tempfile::tempdir().unwrap();
    let mut bin = Command::cargo_bin("c-compiler-in-rust").unwrap();
    bin.arg("--until").arg("tp3");
    test_utils::test_compiler_output(&mut bin, &inputs, &temp.path().join("app"), &expecteds);
}

