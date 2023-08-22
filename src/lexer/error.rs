use crate::location::Loc;

use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexErrorKind {
    #[error("{0}")]
    Any(String),
    #[error("short string not finished, expected matching {0}")]
    UnfinishedShortString(char),
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("hexadecimal digit expected")]
    HexDigitExpected,
    #[error("missing '{{' in \\u{{xxxx}} escape")]
    EscapeUnicodeStart,
    #[error("missing '}}' in \\u{{xxxx}} escape")]
    EscapeUnicodeEnd,
    #[error("invalid unicode value in \\u{{xxxx}} escape")]
    EscapeUnicodeInvalid,
    #[error("\\ddd escape out of 0-255 range")]
    EscapeDecimalTooLarge,
    #[error("invalid escape sequence")]
    InvalidEscape,
    #[error("invalid long string delimiter")]
    InvalidLongStringDelimiter,
    #[error("unfinished long string")]
    UnfinishedLongString,

    #[error("invalid integer number {0}")]
    InvalidNumberInt(String),
    #[error("invalid floating number {0}")]
    InvalidNumberFloat(String),
}

#[derive(Debug, Display)]
#[display(fmt = "{loc}: ERROR - {kind}")]
pub struct LexError<'loc> {
    kind: LexErrorKind,
    loc: Loc<'loc>,
}
impl<'lex> From<LexErrorKind> for LexError<'lex> {
    fn from(kind: LexErrorKind) -> Self {
        Self {
            kind,
            loc: Default::default(),
        }
    }
}

impl<'loc> LexError<'loc> {
    pub fn new(kind: impl Into<LexErrorKind>, loc: Loc<'loc>) -> Self {
        Self {
            kind: kind.into(),
            loc,
        }
    }
    pub fn set_loc(mut self, loc: Loc<'loc>) -> Self {
        self.loc = loc;
        self
    }

    pub fn unfinished_short_string(s: char, loc: Loc<'loc>) -> Self {
        Self {
            kind: LexErrorKind::UnfinishedShortString(s),
            loc,
        }
    }

    pub fn unexpected_char(c: char, loc: Loc<'loc>) -> Self {
        Self {
            kind: LexErrorKind::UnexpectedCharacter(c),
            loc,
        }
    }
}
