use crate::error::CompilerError::UserError;
use crate::lexical_analysis::lexeme::Lexeme;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{
    CharacterConstant, Comment, HeaderName, Identifier, Other, PPNumber, Punctuator, StringLiteral,
    Whitespace,
};
use crate::translation_phases::translation_phase_3::preprocessing_tokens::PreprocessingTokens;
use crate::translation_phases::translation_phase_3::translation_phase_3;
use itertools::Itertools;
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
    let source = "/* /* abc 123 #include<iostream> 2 + 3, */";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![lexeme!(
            Comment,
            "/* /* abc 123 #include<iostream> 2 + 3, */"
        )],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_unterminated_comment() {
    // GIVEN
    let source = "/* test /* **** * * /";
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
fn test_header_rule() {
    // GIVEN
    let source = "#include <stdio.h> #include \"numeric_limits.h\"";
    // WHEN
    let result = translation_phase_3(source).unwrap();
    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Punctuator, "#"),
            lexeme!(Identifier, "include"),
            lexeme!(Whitespace, " "),
            lexeme!(HeaderName, "<stdio.h>"),
            lexeme!(Whitespace, " "),
            lexeme!(Punctuator, "#"),
            lexeme!(Identifier, "include"),
            lexeme!(Whitespace, " "),
            lexeme!(HeaderName, "\"numeric_limits.h\""),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_header_names() {
    // GIVEN
    let source = "\
        #include <a>\
        #include <stdio.h>\
        #include <path/to/file.hpp>\
        #include <A-Z_0.9+->\
        #include \"a\"\
        #include \"stdio.h\"\
        #include \"path/to/file.hpp\"\
        #include \"A-Z_0.9+->\"\
    ";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(HeaderName, "<a>"),
            lexeme!(HeaderName, "<stdio.h>"),
            lexeme!(HeaderName, "<path/to/file.hpp>"),
            lexeme!(HeaderName, "<A-Z_0.9+->"),
            lexeme!(HeaderName, "\"a\""),
            lexeme!(HeaderName, "\"stdio.h\""),
            lexeme!(HeaderName, "\"path/to/file.hpp\""),
            lexeme!(HeaderName, "\"A-Z_0.9+->\""),
        ],
    };

    assert!(
        expected
            .tokens
            .iter()
            .all(|token| { result.tokens.iter().contains(token) })
    );
}

#[test]
fn test_invalid_headers() {
    // GIVEN
    let source = "\
        #define <a>\
        #define \"literal\"\
        #include 'char_const.h'\
        #include \"new_line
        \"\
        #include <non_source_char_@>\
    ";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    assert!(
        result
            .tokens
            .iter()
            .all(|token| { token.kind != HeaderName })
    );
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
fn test_1char_punctuators() {
    // GIVEN
    let source = "[](){}.&*+-~!/%<>^|?;:,#=";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    assert!(
        result
            .tokens
            .iter()
            .all(|token| { token.kind == Punctuator && token.text.len() == 1 })
    );
}

#[test]
fn test_2char_punctuators() {
    // GIVEN
    let source = "->++--<<>><=>===!=&&||*=/=%=+=-=&=^=|=##";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    assert!(
        result
            .tokens
            .iter()
            .all(|token| { token.kind == Punctuator && token.text.len() == 2 })
    );
}

#[test]
fn test_other_punctuators() {
    // GIVEN
    let source = "...sizeof<<=>>=";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(Punctuator, "..."),
            lexeme!(Punctuator, "sizeof"),
            lexeme!(Punctuator, "<<="),
            lexeme!(Punctuator, ">>="),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_character_constant() {
    // GIVEN
    let source = "'a' 'abc123' '!#%&()*+,-./:;<=>?[]^_{|}~' L'Z' \
        '\\n\\t\\r\\b\\f\\v\\a\\\\\\'\\\"\\?' '\\0\\7\\77\\123\\400' '\\x0\\xA\\x1f\\xDEADBEEF' \
        'a\\nb\\123c\\xFF' 'Az09_' L'Hello\\nWorld' '\"'";

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
            lexeme!(Whitespace, " "),
            lexeme!(CharacterConstant, "'\"'"),
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
fn test_string_literal() {
    // GIVEN
    let source = "\"a\" \"abc123\" \"!#%&()*+,-./:;<=>?[]^_{|}~\" L\"Z\" \
        \"\\n\\t\\r\\b\\f\\v\\a\\\\\\\"\\\"\\?\" \"\\0\\7\\77\\123\\400\" \"\\x0\\xA\\x1f\\xDEADBEEF\" \
        \"a\\nb\\123c\\xFF\" \"Az09_\" L\"Hello\\nWorld\" \"'\"";

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    let expected = PreprocessingTokens {
        tokens: vec![
            lexeme!(StringLiteral, "\"a\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"abc123\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"!#%&()*+,-./:;<=>?[]^_{|}~\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "L\"Z\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"\\n\\t\\r\\b\\f\\v\\a\\\\\\\"\\\"\\?\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"\\0\\7\\77\\123\\400\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"\\x0\\xA\\x1f\\xDEADBEEF\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"a\\nb\\123c\\xFF\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"Az09_\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "L\"Hello\\nWorld\""),
            lexeme!(Whitespace, " "),
            lexeme!(StringLiteral, "\"'\""),
        ],
    };

    assert_eq!(result, expected);
}

#[test]
fn test_invalid_string_literal() {
    // GIVEN
    let source = r#""" " "abc "\ "\x" "\8" "abc\" L"" L"\x" "foo "\n"#;

    // WHEN
    let result = translation_phase_3(source).unwrap();

    // THEN
    assert!(
        result
            .tokens
            .iter()
            .all(|token| { token.kind != StringLiteral })
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
