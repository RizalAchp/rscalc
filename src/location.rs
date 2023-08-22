use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Loc<'a> {
    source: Option<Cow<'a, str>>,
    line: u32,
    col: u32,
}

impl<'a> Loc<'a> {
    pub fn new(line: u32, col: u32) -> Self {
        Self {
            source: None,
            line,
            col,
        }
    }

    pub fn set_source<S>(mut self, source: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.source = Some(source.into());
        self
    }

    pub fn source(&self) -> Option<&Cow<'a, str>> {
        self.source.as_ref()
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn inc_col(&mut self) {
        self.col += 1;
    }

    pub fn inc_line(&mut self) {
        self.line += 1;
    }

    pub fn set_col(&mut self, col: u32) {
        self.col = col;
    }
    pub fn set_line(&mut self, line: u32) {
        self.line = line;
    }
}

impl<'a> Display for Loc<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.source {
            write!(f, "{s}:{}:{}", self.line, self.col)
        } else {
            write!(f, "{}:{}", self.line, self.col)
        }
    }
}
