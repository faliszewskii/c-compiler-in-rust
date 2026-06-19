use std::env::current_dir;

mod test_utils;

#[test]
fn test_translation_phase_1() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp1.c")];
    let expecteds = [current_dir()
        .unwrap()
        .join("tests/resources/expected/tp1/tp1.tp1")];

    let temp = tempfile::tempdir().unwrap();
    test_utils::test_compiler_output(&inputs, &temp.path().join("app"), &expecteds, "tp1");
}

#[test]
fn test_translation_phase_2() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp2.c")];
    let expecteds = [
        current_dir()
            .unwrap()
            .join("tests/resources/expected/tp2/tp2.tp1"),
        current_dir()
            .unwrap()
            .join("tests/resources/expected/tp2/tp2.tp2"),
    ];

    let temp = tempfile::tempdir().unwrap();
    test_utils::test_compiler_output(&inputs, &temp.path().join("app"), &expecteds, "tp2");
}

#[test]
fn test_translation_phase_3() {
    let inputs = [current_dir().unwrap().join("tests/resources/input/tp3.c")];
    let expecteds = [
        current_dir()
            .unwrap()
            .join("tests/resources/expected/tp3/tp3.tp1"),
        current_dir()
            .unwrap()
            .join("tests/resources/expected/tp3/tp3.tp2"),
        current_dir()
            .unwrap()
            .join("tests/resources/expected/tp3/tp3.tp3"),
    ];

    let temp = tempfile::tempdir().unwrap();
    test_utils::test_compiler_output(&inputs, &temp.path().join("app"), &expecteds, "tp3");
}
