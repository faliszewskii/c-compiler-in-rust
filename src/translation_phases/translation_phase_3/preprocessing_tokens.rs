use crate::lexical_analysis::lexeme::Lexeme;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct PreprocessingTokens {
    pub tokens: Vec<Lexeme<PreprocessingLexemeKind>>,
}

impl Debug for PreprocessingTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for lexeme in &self.tokens {
            match lexeme.kind {
                PreprocessingLexemeKind::Whitespace => {
                    if lexeme.text == "\n" {
                        writeln!(f)?;
                    }
                }
                _ => write!(f, "{} ", lexeme)?,
            }
        }
        Ok(())
    }
}

impl Display for PreprocessingTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for lexeme in &self.tokens {
            write!(f, "{} ", lexeme)?;
            if PreprocessingLexemeKind::Whitespace == lexeme.kind && lexeme.text == "\n" {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
