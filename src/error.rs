use std::result::Result as StdResult;

use thiserror::Error;

use crate::lexer::error::LexError;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("{0}")]
    Any(String),
    #[error("{0} - ParsingError")]
    ParsingError(String),
    #[error("{0} - LexingError")]
    LexingError(LexError<'a>),

    #[error("IoError - {0}")]
    IoError(#[from] std::io::Error),
}

impl<'err> Error<'err> {
    pub fn any(s: impl ToString) -> Self {
        Self::Any(s.to_string())
    }
    pub fn parsing_error(s: impl ToString) -> Self {
        Self::ParsingError(s.to_string())
    }
    pub fn lexing_error(s: impl Into<LexError<'err>>) -> Self {
        Self::LexingError(s.into())
    }
}

pub type Result<'err, T> = StdResult<T, Error<'err>>;
