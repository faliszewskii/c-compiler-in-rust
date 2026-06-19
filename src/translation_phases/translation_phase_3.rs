use crate::error::CompilerError;
use crate::error::CompilerError::UserError;
use crate::lexical_analysis::lexeme::Lexeme;
use crate::lexical_analysis::naive_lexer::lex;
use crate::lexical_analysis::naive_state_machine::LexerStateMachine;
pub(crate) use crate::translation_phases::translation_phase_3::lexer_transition_rules::{
    PreprocessingLexemeKind, RULE_SET,
};
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::{
    Comment, UnclosedComment, Whitespace,
};

pub fn translation_phase_3(
    source: &str,
) -> Result<Vec<Lexeme<PreprocessingLexemeKind>>, CompilerError> {
    Ok(merge_whitespaces(replace_comments(
        lex_preprocessing_tokens(source)?,
    )))
}

fn lex_preprocessing_tokens(
    source: &str,
) -> Result<Vec<Lexeme<PreprocessingLexemeKind>>, CompilerError> {
    let rule_set = RULE_SET;
    let mut machine = LexerStateMachine {
        rules: Vec::from(rule_set),
    };
    let tokens = lex(source, &mut machine)?;
    if let Some(last_token) = tokens.last()
        && last_token.kind == UnclosedComment
    {
        Err(UserError("Unterminated comment".to_string()))?
    }
    Ok(tokens)
}

fn replace_comments(
    tokens: Vec<Lexeme<PreprocessingLexemeKind>>,
) -> Vec<Lexeme<PreprocessingLexemeKind>> {
    tokens
        .iter()
        .map(|x| {
            if x.kind == Comment {
                Lexeme {
                    kind: Whitespace,
                    text: " ".to_string(),
                }
            } else {
                x.clone()
            }
        })
        .collect::<Vec<_>>()
}

fn merge_whitespaces(
    tokens: Vec<Lexeme<PreprocessingLexemeKind>>,
) -> Vec<Lexeme<PreprocessingLexemeKind>> {
    let mut is_previous_a_whitespace = false;
    tokens
        .iter()
        .filter_map(|x| {
            if x.kind == Whitespace && x.text != "\n" {
                if is_previous_a_whitespace {
                    return None;
                }
                is_previous_a_whitespace = true;
                Some(Lexeme {
                    kind: Whitespace,
                    text: " ".to_string(),
                })
            } else {
                is_previous_a_whitespace = false;
                Some(x.clone())
            }
        })
        .collect::<Vec<_>>()
}

mod lexer_transition_rules;
pub(crate) mod preprocessing_tokens_display;
#[cfg(test)]
mod tests;
