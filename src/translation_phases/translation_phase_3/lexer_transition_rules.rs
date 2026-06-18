use crate::lexical_analysis::naive_state_machine::TransitionRule;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind::Identifier;
use const_format::formatcp;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{CharacterConstant, Comment, HeaderName, Other, PPNumber, Punctuator, StringLiteral, UnclosedComment, Whitespace};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub(crate) enum PreprocessingLexemeKind {
    Comment,
    UnclosedComment,
    HeaderName,
    Identifier,
    PPNumber,
    CharacterConstant,
    StringLiteral,
    Punctuator,
    Other,
    Whitespace,
}

type Rule = TransitionRule<PreprocessingLexemeKind>;

macro_rules! rule {
    ($pattern: expr, $outputs: expr) => {
        Rule {
            pattern: $pattern,
            outputs: $outputs,
        }
    };
}

// white space
const COMMENT: &str = "(//.*)";
const COMMENT_RULE: Rule = rule!(COMMENT, &[Comment]);
const COMMENT_BLOCK: &str = r"(/\*.*\*/)";
const COMMENT_BLOCK_RULE: Rule = rule!(COMMENT_BLOCK, &[Comment]);
const COMMENT_UNCLOSED: &str = r"(/\*(?:[^*]|\*+[^*/])*)";
const COMMENT_UNCLOSED_RULE: Rule = rule!(COMMENT_UNCLOSED, &[UnclosedComment]);
const WHITE_SPACE: &str = r"(\t|\n|\v|\f|\r| )";
const WHITE_SPACE_RULE: Rule = rule!(WHITE_SPACE, &[Whitespace]);

// source character set
const SOURCE_CHAR_SET: &str = r##"[A-Za-z0-9!"#%&'\(\)\*\+,\-\./:;<=>\?\[\\\]\^_\{\|\}~]"##;

// header name
const H_CHAR: &str = formatcp!(r"[{SOURCE_CHAR_SET}--[>\n]]");
const H_CHAR_SEQUENCE: &str = formatcp!(r"{H_CHAR}+");
const Q_CHAR: &str = formatcp!(r#"[{SOURCE_CHAR_SET}--["\n]]"#);
const Q_CHAR_SEQUENCE: &str = formatcp!(r"{Q_CHAR}+");
const HEADER_NAME: &str =
    formatcp!(r#"(#)(include)([\t|\n|\v|\f|\r| ]+)(<{H_CHAR_SEQUENCE}>|"{Q_CHAR_SEQUENCE}")"#);
const HEADER_NAME_RULE: Rule = rule!(
    HEADER_NAME,
    &[Punctuator, Identifier, Whitespace, HeaderName]
);

// identifier
const DIGIT: &str = r"[0-9]";
const NON_DIGIT: &str = r"[_a-zA-Z]";
const IDENTIFIER: &str = formatcp!(r"({NON_DIGIT}(?:{NON_DIGIT}|{DIGIT})*)");
const IDENTIFIER_RULE: Rule = rule!(IDENTIFIER, &[Identifier]);

// pp-number
const PP_NUMBER: &str = formatcp!(r"(\.?{DIGIT}(?:e[\+\-]|E[\+\-]|{NON_DIGIT}|{DIGIT}|\.)*)");
const PP_NUMBER_RULE: Rule = rule!(PP_NUMBER, &[PPNumber]);

// escape sequence
const SIMPLE_ESC_SEQ: &str = r#"(?:\\'|\\"|\\\?|\\\\|\\a|\\b|\\f|\\n|\\r|\\t|\\v)"#;
const OCTAL_ESC_SEQ: &str = r"\\[0-7]{1,3}";
const HEXADECIMAL_ESC_SEQ: &str = r"\\x[0-9A-Fa-f]+";
const ESCAPE_SEQUENCE: &str = formatcp!(r"{SIMPLE_ESC_SEQ}|{OCTAL_ESC_SEQ}|{HEXADECIMAL_ESC_SEQ}");

// character constant
const C_CHAR: &str = formatcp!(r"(?:[{SOURCE_CHAR_SET}--['\\\n]]|{ESCAPE_SEQUENCE})");
const C_CHAR_SEQUENCE: &str = formatcp!(r"{C_CHAR}+");
const CHARACTER_CONSTANT: &str = formatcp!(r"(L?'{C_CHAR_SEQUENCE}')");
const CHARACTER_CONSTANT_RULE: Rule = rule!(CHARACTER_CONSTANT, &[CharacterConstant]);

// string literal
const S_CHAR: &str = formatcp!(r#"(?:[{SOURCE_CHAR_SET}--["\\\n]]|{ESCAPE_SEQUENCE})"#);
const S_CHAR_SEQUENCE: &str = formatcp!(r"{S_CHAR}+");
const STRING_LITERAL: &str = formatcp!(r#"(L?"{S_CHAR_SEQUENCE}")"#);
const STRING_LITERAL_RULE: Rule = rule!(STRING_LITERAL, &[StringLiteral]);

// punctuator
const ONE_CHAR_PUNCTUATOR: &str = r"[\[\]\(\)\{\}\.&\*\+\-~!/%<=>\^\|\?;:,#]";
const MUL_CHAR_PUNCTUATOR: &str = r"(?:sizeof|\.\.\.|<<=|>>=|\->|\+\+|\-\-|<<|>>|<=|>=|==|!=|&&|\|\||\*=|/=|%=|\+=|\-=|&=|\^=|\|=|##)";
const PUNCTUATOR: &str = formatcp!(r"({MUL_CHAR_PUNCTUATOR}|{ONE_CHAR_PUNCTUATOR})");
const PUNCTUATOR_RULE: Rule = rule!(PUNCTUATOR, &[Punctuator]);

// each non-white-space character that cannot be one of the above
const OTHER_RULE: Rule = rule!(r"(.)", &[Other]);

pub(crate) const RULE_SET: &[Rule] = &[
    COMMENT_RULE,
    COMMENT_BLOCK_RULE,
    COMMENT_UNCLOSED_RULE,
    WHITE_SPACE_RULE,
    HEADER_NAME_RULE,
    PUNCTUATOR_RULE,
    IDENTIFIER_RULE,
    PP_NUMBER_RULE,
    CHARACTER_CONSTANT_RULE,
    STRING_LITERAL_RULE,
    OTHER_RULE,
];
