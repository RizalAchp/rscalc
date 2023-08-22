pub mod error;
pub mod token;

#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::{iter::Peekable, str::Chars};

use crate::location::Loc;

use crate::error::{Error, Result};
use error::LexError;
use token::Token;

use self::error::LexErrorKind;
use crate::util::{is_ident, parse_hex_integer};

#[derive(Debug)]
pub struct Lexer<'l> {
    curr_char: char,
    loc: Loc<'l>,
    buf: String,
    source: Peekable<Chars<'l>>,
}

impl<'l> Lexer<'l> {
    const NULL_BYTE_CHAR: char = '\0';

    #[inline]
    pub fn new(s: &'l str) -> Lexer<'l> {
        let mut s = Self {
            curr_char: '\0',
            loc: Loc::new(0, 0),
            buf: String::new(),
            source: s.chars().peekable(),
        };
        s.read_char();
        s
    }

    #[inline]
    pub fn set_source_path(mut self, path: impl Into<Cow<'l, str>>) -> Self {
        self.loc = self.loc.set_source(path);
        self
    }
}

impl<'l> Lexer<'l> {
    #[inline]
    #[allow(unused)]
    pub fn reset(&mut self) {}

    #[inline]
    fn on_newline(&mut self) {
        if matches!(self.curr_char, '\n' | '\r') {
            self.loc.inc_line();
            self.loc.set_col(0);
        } else {
            self.loc.inc_col();
        }
    }

    #[allow(unused)]
    #[inline]
    fn read_char_if(&mut self, pred: impl FnOnce(&char) -> bool) -> Option<&char> {
        match self.source.next_if(pred) {
            Some(s) => {
                self.on_newline();
                self.curr_char = s;
                Some(&self.curr_char)
            }
            None => None,
        }
    }
    #[inline]
    fn read_char(&mut self) -> &char {
        self.on_newline();
        self.curr_char = self.source.next().unwrap_or(Self::NULL_BYTE_CHAR);
        &self.curr_char
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.curr_char.is_whitespace() {
            dbg!(&self.curr_char);
        }
    }

    fn read_ident(&mut self) -> Token<'l> {
        self.buf.clear();
        self.buf.push(self.curr_char);
        while self
            .source
            .peek()
            .filter(|s| s.is_ascii_alphanumeric() || matches!(s, '_'))
            .is_some()
        {
            self.read_char();
            self.buf.push(self.curr_char);
        }
        // dbg!(&self.buf, self.curr_char);

        match self.buf.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "elif" => Token::Elif,
            "return" => Token::Return,
            "while" => Token::While,
            "for" => Token::For,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "struct" => Token::Struct,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::ident(self.buf.clone()),
        }
    }

    fn read_num(&mut self) -> Result<'l, Token<'l>> {
        let mut s = String::with_capacity(16);
        let mut has_radix = false;
        let mut is_float = false;

        let is_hex = if matches!(self.curr_char, '0') {
            s.push(self.curr_char);
            if matches!(self.source.peek(), Some('x') | Some('X')) {
                s.push(*self.read_char());
                self.read_char();
                true
            } else {
                false
            }
        } else {
            false
        };

        'num_loop: loop {
            match self.curr_char {
                '0'..='9' | 'a'..='f' => s.push(self.curr_char),
                'A'..='F' => s.push(self.curr_char.to_ascii_lowercase()),
                '-' | '+' if matches!(s.chars().last(), Some('e')) => {
                    s.push(self.curr_char);
                    is_float = true;
                }
                '.' if !has_radix => {
                    s.push(self.curr_char);
                    has_radix = true;
                    is_float = true;
                }
                '.' => {
                    self.read_char();
                    return Err(Error::lexing_error(LexError::new(
                        LexErrorKind::InvalidNumberInt(format!(
                            "misplace decimal radix '{}'",
                            self.curr_char
                        )),
                        self.loc.clone(),
                    )));
                }
                '_' | ',' => {}
                _ => break 'num_loop,
            }
            self.read_char();
        }
        if has_radix || is_float {
            return s.parse::<f64>().map(Token::Float).map_err(|err| {
                Error::lexing_error(LexError::new(
                    LexErrorKind::InvalidNumberFloat(format!("number: {s} - {err}")),
                    self.loc.clone(),
                ))
            });
        }
        if is_hex {
            return parse_hex_integer(s).map(Token::Int).map_err(|err| {
                Error::lexing_error(LexError::new(
                    LexErrorKind::InvalidNumberInt(err.to_string()),
                    self.loc.clone(),
                ))
            });
        }
        s.parse::<u64>().map(Token::Int).map_err(|err| {
            Error::lexing_error(LexError::new(
                LexErrorKind::InvalidNumberInt(format!("{s} - {err}")),
                self.loc.clone(),
            ))
        })
    }
}

impl<'lit> Iterator for Lexer<'lit> {
    type Item = Result<'lit, Token<'lit>>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! tok {
            ($t:expr) => {{
                self.read_char();
                $t
            }};

            (NEXT_TOKEN => ($($c:literal THEN $t:expr),* $(,)?) OR $def:expr) => {{
                match self.source.peek() {
                    $(Some($c) => tok!($t),)*
                    _ => $def,
                }
            }};
        }

        self.skip_whitespace();
        let token = match self.curr_char {
            '{' => Token::LSquirly,
            '(' => Token::LParen,
            '[' => Token::LBracket,
            '}' => Token::RSquirly,
            ')' => Token::RParen,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            '!' => tok!(NEXT_TOKEN => ('=' THEN Token::Ne) OR Token::Not),
            '%' => tok!(NEXT_TOKEN => ('=' THEN Token::RemAssign) OR Token::Rem),
            '+' => tok!(NEXT_TOKEN => ('=' THEN Token::AddAssign) OR Token::Add),
            '~' => {
                tok!(NEXT_TOKEN => ('=' THEN Token::BitNotXorAssign) OR Token::BitNotXor)
            }

            '*' => {
                tok!(NEXT_TOKEN => ('*' THEN Token::Pow, '=' THEN Token::MulAssign) OR Token::Mul)
            }
            '=' => {
                tok!(NEXT_TOKEN => ('=' THEN Token::Eq,  '>' THEN Token::FatArrow) OR Token::Assign)
            }
            '-' => {
                tok!(NEXT_TOKEN => ('>' THEN Token::Arrow, '=' THEN Token::SubAssign) OR Token::Sub)
            }
            ':' => {
                tok!(NEXT_TOKEN => (':' THEN Token::Decl,'=' THEN Token::DeclAssign) OR Token::Colon)
            }
            '|' => {
                tok!(NEXT_TOKEN => ('=' THEN Token::BitOrAssign, '|' THEN Token::LogicOr) OR Token::BitOr)
            }
            '&' => {
                tok!(NEXT_TOKEN => ('=' THEN Token::BitAndAssign, '&' THEN Token::LogicAnd) OR Token::BitAnd)
            }
            '.' => {
                tok!(NEXT_TOKEN => ('.' THEN tok!( NEXT_TOKEN => ('=' THEN Token::RangeInc) OR Token::Range)) OR Token::Dot)
            }
            '<' => {
                tok!(NEXT_TOKEN => ('<' THEN tok!( NEXT_TOKEN => ('=' THEN Token::ShlAssign) OR Token::Shl),'=' THEN Token::Lte) OR Token::Lt)
            }
            '>' => {
                tok!(NEXT_TOKEN => ('>' THEN tok!( NEXT_TOKEN => ('=' THEN Token::ShrAssign) OR Token::Shr), '=' THEN Token::Gte) OR Token::Gt)
            }
            '/' => {
                tok!(NEXT_TOKEN => ('/' THEN tok!( NEXT_TOKEN => ('=' THEN Token::IDivAssign) OR Token::IDiv),'=' THEN Token::DivAssign) OR Token::Div)
            }
            '^' => {
                tok!(NEXT_TOKEN => ('^' THEN tok!( NEXT_TOKEN => ('=' THEN Token::BitXorAssign) OR Token::BitXor),'=' THEN Token::PowAssign) OR Token::Pow)
            }

            '0'..='9' => match self.read_num() {
                Ok(k) => k,
                Err(err) => {
                    self.read_char();
                    return Some(Err(err));
                }
            },
            c if is_ident(&c) => self.read_ident(),

            Self::NULL_BYTE_CHAR => return None,

            _ => {
                let r = Some(Err(Error::lexing_error(LexError::new(
                    LexErrorKind::UnexpectedCharacter(self.curr_char),
                    self.loc.clone(),
                ))));
                self.read_char();
                return r;
            }
        };
        self.read_char();
        Some(Ok(token))
    }
}
