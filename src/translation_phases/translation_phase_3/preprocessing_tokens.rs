use crate::lexical_analysis::lexeme::Lexeme;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct PreprocessingTokens {
    pub tokens: Vec<Lexeme<PreprocessingLexemeKind>>,
}

impl Debug for PreprocessingTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for PreprocessingTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = &mut self.tokens.iter().peekable();
        while let Some(lexeme) = iter.next() {
            write!(f, "{}", lexeme)?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
            if PreprocessingLexemeKind::Whitespace == lexeme.kind && lexeme.text == "\n" {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
