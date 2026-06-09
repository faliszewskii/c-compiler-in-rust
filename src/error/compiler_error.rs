use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CompilerError {
    UserError(String),
    Internal(anyhow::Error),
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::UserError(msg) => write!(f, "{}", msg),
            CompilerError::Internal(error) => write!(f, "{}", error),
        }
    }
}

impl Error for CompilerError {}