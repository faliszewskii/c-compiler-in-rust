use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexeme<LexemeKind> {
    pub kind: LexemeKind,
    pub text: String,
}

impl<LexemeKind: Display> Display for Lexeme<LexemeKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.kind, self.text)
    }
}