use derive_more::Display;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, PartialOrd, Display)]
pub enum Token<'lit> {
    #[display(fmt = "<IllegalToken>")]
    Illegal,
    #[display(fmt = "<EOF>")]
    Eof,
    #[display(fmt = "<If>")]
    If,
    #[display(fmt = "<Else>")]
    Else,
    #[display(fmt = "<ElseIf>")]
    Elif,
    #[display(fmt = "<Return>")]
    Return,
    #[display(fmt = "<For>")]
    For,
    #[display(fmt = "<While>")]
    While,
    #[display(fmt = "<Break>")]
    Break,
    #[display(fmt = "<Continue>")]
    Continue,
    #[display(fmt = "<Struct>")]
    Struct,

    #[display(fmt = "<True>")]
    True,
    #[display(fmt = "<False>")]
    False,

    #[display(fmt = "'['")]
    LBracket,
    #[display(fmt = "']'")]
    RBracket,
    #[display(fmt = "'('")]
    LParen,
    #[display(fmt = "')'")]
    RParen,
    #[display(fmt = "'{{'")]
    LSquirly,
    #[display(fmt = "'}}'")]
    RSquirly,

    #[display(fmt = "'=='")]
    Eq,
    #[display(fmt = "'!='")]
    Ne,
    #[display(fmt = "'<'")]
    Lt,
    #[display(fmt = "'<='")]
    Lte,
    #[display(fmt = "'>='")]
    Gt,
    #[display(fmt = "'>='")]
    Gte,
    #[display(fmt = "'&&'")]
    LogicAnd,
    #[display(fmt = "'||'")]
    LogicOr,

    #[display(fmt = "'!'")]
    Not,

    #[display(fmt = "'+'")]
    Add,
    #[display(fmt = "'+='")]
    AddAssign,
    #[display(fmt = "'-'")]
    Sub,
    #[display(fmt = "'-='")]
    SubAssign,
    #[display(fmt = "'%'")]
    Rem,
    #[display(fmt = "'%='")]
    RemAssign,
    #[display(fmt = "'*'")]
    Mul,
    #[display(fmt = "'*='")]
    MulAssign,
    #[display(fmt = "'^'")]
    Pow,
    #[display(fmt = "'^='")]
    PowAssign,
    #[display(fmt = "'/'")]
    Div,
    #[display(fmt = "'/='")]
    DivAssign,
    #[display(fmt = "'//'")]
    IDiv,
    #[display(fmt = "'//='")]
    IDivAssign,
    #[display(fmt = "'>>'")]
    Shr,
    #[display(fmt = "'>>='")]
    ShrAssign,
    #[display(fmt = "'<<'")]
    Shl,
    #[display(fmt = "'<<='")]
    ShlAssign,
    #[display(fmt = "'|'")]
    BitOr,
    #[display(fmt = "'|='")]
    BitOrAssign,
    #[display(fmt = "'&'")]
    BitAnd,
    #[display(fmt = "'&='")]
    BitAndAssign,
    #[display(fmt = "'^^'")]
    BitXor,
    #[display(fmt = "'^^='")]
    BitXorAssign,
    #[display(fmt = "'~'")]
    BitNotXor,
    #[display(fmt = "'~='")]
    BitNotXorAssign,

    #[display(fmt = "'='")]
    Assign,
    #[display(fmt = "':'")]
    Colon, // =
    #[display(fmt = "':='")]
    DeclAssign, // =
    #[display(fmt = "'::'")]
    Decl, // =

    #[display(fmt = "';'")]
    SemiColon, // =
    #[display(fmt = "'.'")]
    Dot, // =
    #[display(fmt = "'..'")]
    Range, // =
    #[display(fmt = "'..='")]
    RangeInc, // =
    #[display(fmt = "','")]
    Comma, // =
    #[display(fmt = "'->'")]
    Arrow, // =
    #[display(fmt = "'=>'")]
    FatArrow, // =
    #[display(fmt = "'//'")]
    Comment, // =

    #[display(fmt = "`{_0}`")]
    Int(u64),
    #[display(fmt = "`{_0}`")]
    Float(f64),
    #[display(fmt = "`{_0}`")]
    Ident(Cow<'lit, str>),
    #[display(fmt = "\"{_0}\"")]
    Str(Cow<'lit, str>),
}

impl<'lit> Token<'lit> {
    #[inline]
    pub fn ident(i: impl Into<Cow<'lit, str>>) -> Self {
        Self::Ident(i.into())
    }
}
