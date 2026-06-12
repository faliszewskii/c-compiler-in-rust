use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_non_existing_file() {
    Command::cargo_bin("c-compiler-in-rust")
        .unwrap()
        .arg("non-existing-file.c")
        .assert()
        .stdout("")
        .stderr(predicate::str::contains(
            "Could not read from file non-existing-file.c",
        ))
        .code(1)
        .failure();
}
