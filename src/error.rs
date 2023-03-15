use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

#[derive(Debug)]
pub enum Error {
    Any(String),
    ParsingError(String),
    BadToken(char),
    MismatchParams,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any(err) => write!(f, "ERROR[PARSING]: {err}"),
            Self::ParsingError(err) => write!(f, "ERROR[PARSING]: {err}"),
            Self::BadToken(tok) => write!(f, "ERROR[BAD TOKEN] = `{tok}`"),
            Self::MismatchParams => write!(f, "ERROR[MISMATCH PARAMS]"),
        }
    }
}

pub type Result<T> = StdResult<T, Error>;
