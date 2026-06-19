use crate::lexical_analysis::lexeme::Lexeme;
use crate::translation_phases::translation_phase_3::PreprocessingLexemeKind::Whitespace;
use crate::translation_phases::translation_phase_3::lexer_transition_rules::PreprocessingLexemeKind;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct PreprocessingTokensDisplay<'a> {
    pub tokens: &'a Vec<Lexeme<PreprocessingLexemeKind>>,
}

impl Debug for PreprocessingTokensDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for PreprocessingTokensDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = &mut self.tokens.iter().peekable();
        while let Some(lexeme) = iter.next() {
            write!(f, "{}", lexeme)?;
            if iter.peek().is_some() && !(lexeme.kind == Whitespace && lexeme.text == "\n") {
                write!(f, " ")?;
            }
            if Whitespace == lexeme.kind && lexeme.text == "\n" {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
