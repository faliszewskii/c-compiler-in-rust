use crate::lexical_analysis::lexeme::Lexeme;
use crate::error::CompilerError::UserError;
use crate::translation_phases::translation_phase_3::preprocessing_tokens::PreprocessingTokens;
use crate::translation_phases::translation_phase_3::translation_phase_3;
use predicates::Predicate;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{Identifier, PPNumber, Whitespace};

macro_rules! lexeme {
    ($kind: ident, $text: literal) => {
        Lexeme {
            kind: $kind,
            text: String::from($text),
        }
    };
}

#[test]
fn test_block_comment() {
    // GIVEN
    let source = "/* abc 123 #include<iostream> 2 + 3, */";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens { tokens: vec!(
        lexeme!(Whitespace, "/*"),
        lexeme!(Whitespace, " abc 123 #include<iostream> 2 + 3, */"),
    ) };
    assert_eq!(result, expected);
}

#[test]
fn test_unterminated_comment() {
    // GIVEN
    let source = "/*";
    // WHEN
    let result = translation_phase_3(source);
    // THEN
    let expected = "Unterminated comment";
    assert!(result.is_err());
    let result = result.unwrap_err();
    match result {
        UserError(msg) => assert!(predicates::str::contains(expected).eval(msg.as_str())),
        _ => {
            panic!("Wrong error")
        }
    }
}

#[test]
fn test_whitespace() {
    // GIVEN
    let source = "\t\n\r ";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens { tokens: vec![
        lexeme!(Whitespace, "\t"),
        lexeme!(Whitespace, "\n"),
        lexeme!(Whitespace, "\r"),
        lexeme!(Whitespace, " "),
    ], };
    assert_eq!(result, expected);
}

#[test]
fn test_identifier() {
    // GIVEN
    let source = "_abc ZYX f4_4";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens { tokens: vec![
        lexeme!(Identifier, "_abc"),
        lexeme!(Whitespace, " "),
        lexeme!(Identifier, "ZYX"),
        lexeme!(Whitespace, " "),
        lexeme!(Identifier, "f4_4"),
    ], };
    assert_eq!(result, expected);
}

#[test]
fn test_pp_number() {
    // GIVEN
    let source = ".123.456e-e-. 1..E+3.foo 0JBK";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens { tokens: vec![
        lexeme!(PPNumber, ".123.456e-e-."),
        lexeme!(Whitespace, " "),
        lexeme!(PPNumber, "1..E+3.foo"),
        lexeme!(Whitespace, " "),
        lexeme!(PPNumber, "0JBK"),
    ], };
    assert_eq!(result, expected);
}