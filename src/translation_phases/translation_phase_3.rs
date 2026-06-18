use crate::error::CompilerError;
use crate::error::CompilerError::UserError;
use crate::lexical_analysis::naive_lexer::lex;
use crate::lexical_analysis::naive_state_machine::LexerStateMachine;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::UnclosedComment;
pub(crate) use crate::translation_phases::translation_phase_3::lexer_transition_rules::{
    PreprocessingLexemeKind, RULE_SET,
};
use crate::translation_phases::translation_phase_3::preprocessing_tokens::PreprocessingTokens;

pub fn translation_phase_3(source: &str) -> Result<PreprocessingTokens, CompilerError> {
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
    Ok(PreprocessingTokens { tokens })
}

mod lexer_transition_rules;
pub(crate) mod preprocessing_tokens;
#[cfg(test)]
mod tests;
