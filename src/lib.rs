mod error;
mod lev;
mod lexer;
mod location;
mod util;

pub use error::{Error, Result};
pub use lev::{lev, suggest_word};
pub use lexer::Lexer;
