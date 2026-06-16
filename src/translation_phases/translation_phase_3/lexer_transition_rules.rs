use crate::lexical_analysis::naive_state_machine::TransitionRule;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind::Identifier;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingState::{InComment, Normal};
use const_format::formatcp;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{PPNumber, Whitespace};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum PreprocessingLexemeKind {
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
const COMMENT: Rule = rule!(r"(//.*\\n)", Normal, Normal, &[Whitespace]);
const COMMENT_BLOCK_BEGIN: Rule = rule!(r"(/\*)", Normal, InComment, &[Whitespace]);
const COMMENT_BLOCK_END: Rule = rule!(r"(.*\*/)", InComment, Normal, &[Whitespace]);
const WHITE_SPACE: Rule = rule!(r"(\t|\n|\v|\f|\r| )", Normal, Normal, &[Whitespace]);

// identifier
const DIGIT: &str = r"[0-9]";
const NON_DIGIT: &str = r"[_a-zA-Z]";
const IDENTIFIER: &str = formatcp!(r"({NON_DIGIT}(?:{NON_DIGIT}|{DIGIT})*)");
const IDENTIFIER_RULE: Rule = rule!(IDENTIFIER, Normal, Normal, &[Identifier]);

// pp-number
const PP_NUMBER: &str = formatcp!(r"(\.?{DIGIT}(?:e[\+\-]|E[\+\-]|{NON_DIGIT}|{DIGIT}|\.)*)");
const PP_NUMBER_RULE: Rule = rule!(PP_NUMBER, Normal, Normal, &[PPNumber]);

// punctuator
const ONE_CHAR_PUNCTUATOR: &str = r"[\[\]\(\)\{\}\.&\*\+\-~!/%<=>\^\|\?;:,#]";
const MUL_CHAR_PUNCTUATOR: &str = r"(?:\.\.\.|\->|\+\+|\-\-|sizeof|<<|>>|<=|>=|==|!=|&&|\|\||\*=|/=|%=|\+=|\-=|<<=|>>=|&=|\^=|\|=|##)";
const PUNCTUATOR: &str = formatcp!(r"({ONE_CHAR_PUNCTUATOR}|{MUL_CHAR_PUNCTUATOR})");
const PUNCTUATOR_RULE: Rule = rule!(PUNCTUATOR, Normal, Normal, &[Identifier]);


pub(crate) const RULE_SET: &[Rule] = &[
    COMMENT,
    COMMENT_BLOCK_BEGIN,
    COMMENT_BLOCK_END,
    WHITE_SPACE,
    // HEADER_NAME_RULE,
    IDENTIFIER_RULE,
    PP_NUMBER_RULE,
    // CHARACTER_CONSTANT_RULE,
    // STRING_LITERAL_RULE,
    PUNCTUATOR_RULE,
    // OTHER
];
