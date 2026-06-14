use crate::lexical_analysis::naive_state_machine::TransitionRule;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind::Identifier;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingState::{InComment, Normal};
use const_format::formatcp;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::Whitespace;
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

const COMMENT: Rule = rule!(r"(//.*\\n)", Normal, Normal, &[Whitespace]);
const COMMENT_BEGIN: Rule = rule!(r"(/\*)", Normal, InComment, &[Whitespace]);
const COMMENT_END: Rule = rule!(r"(.*\*/)", InComment, Normal, &[Whitespace]);
const WHITE_SPACE: Rule = rule!(r"(\t|\n|\v|\f|\r| )", Normal, Normal, &[Whitespace]);


const DIGIT: &str = r"[0-9]";
const NON_DIGIT: &str = r"[_a-zA-Z]";
const IDENTIFIER: &str = formatcp!(r"({NON_DIGIT}(?:{NON_DIGIT}|{DIGIT})*)");
const IDENTIFIER_RULE: Rule = rule!(IDENTIFIER, Normal, Normal, &[Identifier]);


pub(crate) const RULE_SET: &[Rule] = &[
    COMMENT,
    COMMENT_BEGIN,
    COMMENT_END,
    WHITE_SPACE,
    IDENTIFIER_RULE,
];
