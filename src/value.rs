use crate::{error::Error, lev::suggest_word};
use num_traits::ToPrimitive;
// use std::i64::*;
use std::{
    fmt::{Debug, Display},
    ops::*,
    str::FromStr,
};

pub const VTYPE: [&'static str; 9] = [
    "i32", "u32", "i64", "u64", "f32", "f64", "isize", "usize", "bool",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ty {
    BOOL,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISIZE,
    USIZE,
}
impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::BOOL => write!(f, "bool"),
            Ty::I32 => write!(f, "i32"),
            Ty::U32 => write!(f, "u32"),
            Ty::I64 => write!(f, "i64"),
            Ty::U64 => write!(f, "u64"),
            Ty::F32 => write!(f, "f32"),
            Ty::F64 => write!(f, "f64"),
            Ty::ISIZE => write!(f, "isize"),
            Ty::USIZE => write!(f, "usize"),
        }
    }
}

impl FromStr for Ty {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bool" => Ok(Self::BOOL),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" => Ok(Self::I64),
            "u64" => Ok(Self::U64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "isize" => Ok(Self::ISIZE),
            "usize" => Ok(Self::USIZE),
            other => {
                let (_, word) = suggest_word(other, &VTYPE[..]);
                Err(Error::ParsingError(format!(
                    "Unknown Types: '{other}'\ndid you mean: '{word}'?",
                )))
            }
        }
    }
}

pub struct NumberType {
    ty: Ty,
    val: Box<dyn ToPrimitive>,
}

impl Debug for NumberType {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Number")
            .field("ty", &self.ty)
            .field("val", &self.val.to_i64())
            .finish()
    }
}
impl Default for NumberType {
    #[inline(always)]
    fn default() -> Self {
        Self {
            ty: Ty::ISIZE,
            val: Box::new(0),
        }
    }
}

impl Display for NumberType {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            Ty::BOOL => write!(f, "{}: {}", self.integral() != 0, self.ty),
            t if matches!(t, Ty::F64 | Ty::F32) => write!(f, "{:.2}: {}", self.floating(), self.ty),
            _ => write!(f, "{}: {}", self.floating(), self.ty),
        }
    }
}
impl Clone for NumberType {
    #[inline(always)]
    fn clone(&self) -> Self {
        let Self { ty, val } = self;
        let val = val.to_f64().unwrap_or_default();
        Self {
            ty: ty.clone(),
            val: Box::new(val),
        }
    }
}
impl PartialEq for NumberType {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.integral() == other.integral()
    }
}
impl Eq for NumberType {}

impl PartialOrd for NumberType {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let other = other.floating();
        self.floating().partial_cmp(&other)
    }
}
impl Ord for NumberType {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let other = other.integral();
        self.integral().cmp(&other)
    }
}

impl NumberType {
    pub fn new(ty: Ty, num: impl ToPrimitive + 'static) -> Self {
        Self {
            ty,
            val: Box::new(num),
        }
    }
    #[inline(always)]
    pub fn integral(&self) -> i64 {
        self.val.to_i64().unwrap_or_default()
    }
    pub fn floating(&self) -> f64 {
        self.val.to_f64().unwrap_or_default()
    }
    pub fn is_truthy(&self) -> bool {
        let floating = self.floating();
        f64::is_normal(floating)
    }
}

use paste::paste;
macro_rules! impl_nums_float {
    ($strc:ty => $($traits:ident),* $(,)?) => {
        paste!{
            $(
                impl $traits for $strc {
                    type Output = Self;
                    #[allow(unused)]
                    #[inline(always)]
                    fn [<$traits:lower>](self, rhs: Self) -> Self::Output {
                        let ty = self.ty.max(rhs.ty);
                        Self::new(ty, self.floating().[<$traits:lower>](rhs.floating()))
                    }
                }

                impl [<$traits Assign>] for $strc {
                    #[allow(unused)]
                    #[inline(always)]
                    fn [<$traits:lower _assign>](&mut self, rhs: Self) {
                        let lhs = self.floating();
                        let rhs = rhs.floating();
                        self.val = Box::new(lhs / rhs)
                    }
                }
            )*
        }
    };
}

macro_rules! impl_nums_int {
    ($strc:ty => {$($traits:ident),* $(,)?}) => {
        paste!{
            $(
                impl $traits for $strc {
                    type Output = Self;
                    #[allow(unused)]
                    #[inline(always)]
                    fn [<$traits:lower>](self, rhs: Self) -> Self::Output {
                        let ty = self.ty.max(rhs.ty);
                        Self::new(ty, self.integral().[<$traits:lower>](rhs.integral()))
                    }
                }
                impl [<$traits Assign>] for $strc {
                    #[allow(unused)]
                    #[inline(always)]
                    fn [<$traits:lower _assign>](&mut self, rhs: Self) {
                        let lhs = self.integral();
                        let rhs = rhs.integral();
                        self.val = Box::new(lhs / rhs)
                    }
                }
            )*
        }
    };
}

macro_rules! impl_froms {
    ($st:ident => $($tp:ident),* $(,)?) => {
        paste! {$(
            impl From<$tp> for $st {
                fn from(num: $tp) -> Self {
                    Self {
                        ty: Ty::[<$tp:upper>],
                        val: Box::new(num),
                    }
                }
            }
        )*}
    };
}

impl_froms!( NumberType => i32, u32, i64, u64, isize, usize, f32, f64 );

impl_nums_float!(NumberType =>
    Add,
    Sub,
    Mul,
    Div,
    Rem,
);

impl_nums_int!(NumberType => {
    Shr,
    Shl,
    BitAnd,
    BitOr,
    BitXor
});

#[test]
fn test_value_type() {
    let lhs = NumberType::new(Ty::F64, 138f64);
    let rhs = NumberType::new(Ty::F64, 69f64);
    let res = lhs.mul(rhs);
    assert_eq!(res.integral(), 9522);
}
