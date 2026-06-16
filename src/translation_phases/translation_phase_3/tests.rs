use crate::error::CompilerError::UserError;
use crate::lexical_analysis::lexeme::Lexeme;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{CharacterConstant, Identifier, Other, PPNumber, Whitespace};
use crate::translation_phases::translation_phase_3::preprocessing_tokens::PreprocessingTokens;
use crate::translation_phases::translation_phase_3::translation_phase_3;
use predicates::Predicate;

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
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Whitespace, "/*"),
            lexeme!(Whitespace, " abc 123 #include<iostream> 2 + 3, */"),
        ],
    };
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
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Whitespace, "\t"),
            lexeme!(Whitespace, "\n"),
            lexeme!(Whitespace, "\r"),
            lexeme!(Whitespace, " "),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_identifier() {
    // GIVEN
    let source = "_abc ZYX f4_4";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Identifier, "_abc"),
            lexeme!(Whitespace, " "),
            lexeme!(Identifier, "ZYX"),
            lexeme!(Whitespace, " "),
            lexeme!(Identifier, "f4_4"),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_pp_number() {
    // GIVEN
    let source = ".123.456e-e-. 1..E+3.foo 0JBK";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(PPNumber, ".123.456e-e-."),
            lexeme!(Whitespace, " "),
            lexeme!(PPNumber, "1..E+3.foo"),
            lexeme!(Whitespace, " "),
            lexeme!(PPNumber, "0JBK"),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_character_constant() {
    // GIVEN
    let source = "'a' 'abc123' '!#%&()*+,-./:;<=>?[]^_{|}~' L'Z' \
        '\\n\\t\\r\\b\\f\\v\\a\\\\\\'\\\"\\?' '\\0\\7\\77\\123\\400' '\\x0\\xA\\x1f\\xDEADBEEF' \
        'a\\nb\\123c\\xFF' 'Az09_' L'Hello\\nWorld'";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(CharacterConstant, "'a'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'abc123'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'!#%&()*+,-./:;<=>?[]^_{|}~'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "L'Z'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'\\n\\t\\r\\b\\f\\v\\a\\\\\\'\\\"\\?'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'\\0\\7\\77\\123\\400'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'\\x0\\xA\\x1f\\xDEADBEEF'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'a\\nb\\123c\\xFF'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'Az09_'"),
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "L'Hello\\nWorld'"),
        ],
    };

    assert_eq!(result, expected);
}

#[test]
fn test_invalid_character_constant() {
    // GIVEN
    let source = r"'' ' 'abc '\ '\x' '\8' 'abc\' L'' L'\x' 'foo '\n";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    assert!(
        result
            .tokens
            .iter()
            .all(|token| { token.kind != CharacterConstant })
    );
}


#[test]
fn test_other_rule() {
    // GIVEN
    let source = r"@ $ `";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Other, "@"),
            lexeme!(Whitespace, " "),
            lexeme!(Other, "$"),
            lexeme!(Whitespace, " "),
            lexeme!(Other, "`"),
        ],
    };
    assert_eq!(result, expected);
}