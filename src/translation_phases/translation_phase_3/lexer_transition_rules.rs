use crate::lexical_analysis::naive_state_machine::TransitionRule;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind::Identifier;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingState::{InComment, Normal};
use const_format::formatcp;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{CharacterConstant, Other, PPNumber, StringLiteral, Whitespace};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub(crate) enum PreprocessingLexemeKind {
    HeaderName,
    Identifier,
    PPNumber,
    CharacterConstant,
    StringLiteral,
    Operator,
    Punctuator,
    Other,
    Whitespace,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreprocessingState {
    Normal,
    InComment,
}

type Rule = TransitionRule<PreprocessingState, PreprocessingLexemeKind>;

macro_rules! rule {
    ($pattern: expr, $current_state: path, $next_state: path, $outputs: expr) => {
        Rule {
            pattern: $pattern,
            current_state: $current_state,
            next_state: $next_state,
            outputs: $outputs,
        }
    };
}

// white space
const COMMENT: &str = r"(//.*\\n)";
const COMMENT_RULE: Rule = rule!(COMMENT, Normal, Normal, &[Whitespace]);
const COMMENT_BLOCK_BEGIN: &str = r"(/\*)";
const COMMENT_BLOCK_BEGIN_RULE: Rule = rule!(COMMENT_BLOCK_BEGIN, Normal, InComment, &[Whitespace]);
const COMMENT_BLOCK_END: &str = r"(.*\*/)";
const COMMENT_BLOCK_END_RULE: Rule = rule!(COMMENT_BLOCK_END, InComment, Normal, &[Whitespace]);
const WHITE_SPACE: &str = r"(\t|\n|\v|\f|\r| )";
const WHITE_SPACE_RULE: Rule = rule!(WHITE_SPACE, Normal, Normal, &[Whitespace]);

// header name
const HEADER_NAME: &str = ""; // TODO

// identifier
const DIGIT: &str = r"[0-9]";
const NON_DIGIT: &str = r"[_a-zA-Z]";
const IDENTIFIER: &str = formatcp!(r"({NON_DIGIT}(?:{NON_DIGIT}|{DIGIT})*)");
const IDENTIFIER_RULE: Rule = rule!(IDENTIFIER, Normal, Normal, &[Identifier]);

// pp-number
const PP_NUMBER: &str = formatcp!(r"(\.?{DIGIT}(?:e[\+\-]|E[\+\-]|{NON_DIGIT}|{DIGIT}|\.)*)");
const PP_NUMBER_RULE: Rule = rule!(PP_NUMBER, Normal, Normal, &[PPNumber]);

// character constant
const SIMPLE_ESC_SEQ: &str = r#"(?:\\'|\\"|\\\?|\\\\|\\a|\\b|\\f|\\n|\\r|\\t|\\v)"#;
const OCTAL_ESC_SEQ: &str = r"\\[0-7]{1,3}";
const HEXADECIMAL_ESC_SEQ: &str = r"\\x[0-9A-Fa-f]+";
const ESCAPE_SEQUENCE: &str = formatcp!(r"{SIMPLE_ESC_SEQ}|{OCTAL_ESC_SEQ}|{HEXADECIMAL_ESC_SEQ}");
const SOURCE_CHAR_SET: &str = r##"[A-Za-z0-9!"#%&'\(\)\*\+,\-\./:;<=>\?\[\\\]\^_\{\|\}~]"##;
const C_CHAR: &str = formatcp!(r"(?:[{SOURCE_CHAR_SET}--['\\\n]]|{ESCAPE_SEQUENCE})");
const C_CHAR_SEQUENCE: &str = formatcp!(r"{C_CHAR}+");
const CHARACTER_CONSTANT: &str = formatcp!(r"(L?'{C_CHAR_SEQUENCE}')");
const CHARACTER_CONSTANT_RULE: Rule =
    rule!(CHARACTER_CONSTANT, Normal, Normal, &[CharacterConstant]);

// string literal
const S_CHAR: &str = formatcp!(r#"(?:[{SOURCE_CHAR_SET}--["\\\n]]|{ESCAPE_SEQUENCE})"#);
const S_CHAR_SEQUENCE: &str = formatcp!(r"{S_CHAR}+");
const STRING_LITERAL: &str = formatcp!(r#"(L?"{S_CHAR_SEQUENCE}")"#);
const STRING_LITERAL_RULE: Rule =
    rule!(STRING_LITERAL, Normal, Normal, &[StringLiteral]);

// punctuator
const ONE_CHAR_PUNCTUATOR: &str = r"[\[\]\(\)\{\}\.&\*\+\-~!/%<=>\^\|\?;:,#]";
const MUL_CHAR_PUNCTUATOR: &str = r"(?:\.\.\.|\->|\+\+|\-\-|sizeof|<<|>>|<=|>=|==|!=|&&|\|\||\*=|/=|%=|\+=|\-=|<<=|>>=|&=|\^=|\|=|##)";
const PUNCTUATOR: &str = formatcp!(r"({ONE_CHAR_PUNCTUATOR}|{MUL_CHAR_PUNCTUATOR})");
const PUNCTUATOR_RULE: Rule = rule!(PUNCTUATOR, Normal, Normal, &[Identifier]);

// each non-white-space character that cannot be one of the above
const OTHER_RULE: Rule = rule!(r"(.)", Normal, Normal, &[Other]);

pub(crate) const RULE_SET: &[Rule] = &[
    COMMENT_RULE,
    COMMENT_BLOCK_BEGIN_RULE,
    COMMENT_BLOCK_END_RULE,
    WHITE_SPACE_RULE,
    // HEADER_NAME_RULE,
    IDENTIFIER_RULE,
    PP_NUMBER_RULE,
    CHARACTER_CONSTANT_RULE,
    STRING_LITERAL_RULE,
    PUNCTUATOR_RULE,
    OTHER_RULE,
];
