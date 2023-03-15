use std::io;

mod error;
mod lev;
mod parser;
mod tok;
mod value;
pub use error::{Error, Result};
pub use lev::suggest_word;
pub use tok::{Comparison, Expresions, Operator, Sign, Token};
pub use value::{NumberType, Ty, VTYPE};

pub fn from_str<S: AsRef<str>>(source: S) -> error::Result<Expresions> {
    parser::parse(source)
}

pub fn from_reader<R: io::Read>(mut reader: R) -> error::Result<Expresions> {
    let mut source = String::new();
    reader
        .read_to_string(&mut source)
        .map_err(|x| error::Error::ParsingError(x.to_string()))?;
    parser::parse(source)
}

pub fn from_slice(source: &'_ [u8]) -> error::Result<Expresions> {
    let source = String::from_utf8_lossy(source).to_string();
    parser::parse(source)
}

#[test]
fn test_parse() {
    let mut expressions = parser::parse("(2 * 2 + 4_822) >> 4").unwrap();
    println!("{expressions:?}");

    let evaluate = expressions.eval().unwrap();
    println!("evalueate to {evaluate}");

    let mut expressions = parser::parse("1 == 1").unwrap();
    println!("{expressions:?}");

    let evaluate = expressions.eval().unwrap();
    println!("evalueate to {evaluate}");
}
