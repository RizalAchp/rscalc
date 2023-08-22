#![allow(unused)]

use std::error::Error;
type DynResult<T> = Result<T, Box<dyn Error + 'static>>;

pub(crate) fn parse_hex_integer(s: impl AsRef<str>) -> DynResult<u64> {
    let s = s.as_ref();
    match &s[..2] {
        "0x" | "0Xd" => Ok(u64::from_str_radix(&s[2..], 16)?),
        s => Err(format!("expected prefix '0x' or '0X', got '{s}' for Hex Integer").into()),
    }
}

pub(crate) fn parse_hex_float(s: impl AsRef<str>) -> DynResult<f64> {
    Ok(f64::from_bits(parse_hex_integer(s)?))
}

#[inline(always)]
pub(super) const fn is_newline(c: u8) -> bool {
    matches!(c, b'\n' | b'\r')
}

#[inline(always)]
pub(super) const fn is_space(c: u8) -> bool {
    matches!(c, b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' | b'\x0B')
}

// Is this character a lua alpha, which is A-Z, a-z, and _
#[inline(always)]
pub(super) const fn is_alpha(c: u8) -> bool {
    matches!(c, b'a'..=b'z'|b'A'..=b'Z'|b'_')
}

#[inline(always)]
pub(super) fn is_ident(c: &char) -> bool {
    c.is_alphanumeric() || matches!(c, '_')
}

#[inline(always)]
pub(super) const fn from_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        _ => None,
    }
}

#[inline(always)]
pub(super) const fn is_digit(c: u8) -> bool {
    from_digit(c).is_some()
}

#[inline(always)]
pub(super) const fn from_hex_digit(c: u8) -> Option<u8> {
    match from_digit(c) {
        Some(b'a'..=b'f') => Some(10 + c - b'a'),
        Some(b'A'..=b'F') => Some(10 + c - b'A'),
        Some(c) => Some(c),
        None => None,
    }
}

#[inline(always)]
pub(super) const fn is_hex_digit(c: u8) -> bool {
    from_hex_digit(c).is_some()
}
