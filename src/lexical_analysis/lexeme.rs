use predicates::Predicate;
use predicates::prelude::predicate;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexeme<LexemeKind> {
    pub kind: LexemeKind,
    pub text: String,
}

impl<LexemeKind: Display> Display for Lexeme<LexemeKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_new_line = predicate::str::contains("\n").eval(&self.text);
        write!(
            f,
            "{}({})",
            self.kind,
            if is_new_line { "LF" } else { &self.text }
        )
    }
}
