use crate::error::{Error, Result};
use crate::tok::{Comparison, Expresions, Operator, Sign, Token};
use crate::value::{NumberType, Ty};

pub fn parse<T: AsRef<str>>(expr: T) -> Result<Expresions> {
    let expr = expr.as_ref();
    let expr_len = expr.len();
    let mut chars = expr.chars().collect::<Vec<_>>();
    chars.reverse();

    let mut tokens = Vec::with_capacity(expr_len);
    let mut parens = Vec::new();

    tokens.push(Token::StartTok);
    while let Some(c) = chars.pop() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            ';' => {
                tokens.push(Token::EndTok);
                tokens.push(Token::StartTok);
            }
            '0'..='9' => {
                let mut is_float = false;
                let mut num_str = String::new();
                num_str.push(c);
                'num_lookup: while let Some(tok) = chars.last() {
                    match tok {
                        '_' | '-' => {
                            chars.pop();
                            continue 'num_lookup;
                        }
                        '.' => {
                            num_str.push('.');
                            is_float = true;
                            chars.pop();
                        }
                        x if x.is_numeric() => {
                            num_str.push(*x);
                            chars.pop();
                        }
                        _ => break 'num_lookup,
                    }
                }
                if is_float {
                    let val = num_str.parse::<f64>().unwrap_or_default();
                    tokens.push(Token::Num(NumberType::new(Ty::F64, val)));
                }
                let val = num_str.parse::<i64>().unwrap_or_default();
                tokens.push(Token::Num(NumberType::new(Ty::F64, val)));
            }
            '(' => {
                tokens.push(Token::Bracket(c));
                parens.push(c);
            }
            ')' => {
                tokens.push(Token::Bracket(c));
                if !matches!(parens.pop(), Some('(')) {
                    return Err(Error::MismatchParams);
                }
            }
            '{' => {
                tokens.push(Token::Bracket(c));
                parens.push(c);
            }
            '}' => {
                tokens.push(Token::Bracket(c));
                if !matches!(parens.pop(), Some('(')) {
                    return Err(Error::MismatchParams);
                }
            }
            '[' => {
                tokens.push(Token::Bracket(c));
                parens.push(c);
            }
            ']' => {
                tokens.push(Token::Bracket(c));
                if !matches!(parens.pop(), Some('(')) {
                    return Err(Error::MismatchParams);
                }
            }
            '+' => tokens.push(Token::Op(Operator::Add)),
            '-' => tokens.push(Token::Op(Operator::Sub)),
            '*' => tokens.push(Token::Op(Operator::Mul)),
            '/' => tokens.push(Token::Op(Operator::Div)),
            '%' => tokens.push(Token::Op(Operator::Rem)),
            '|' => tokens.push(Token::Op(Operator::BitOr)),
            '&' => tokens.push(Token::Op(Operator::BitAnd)),
            '^' => tokens.push(Token::Op(Operator::BitXor)),
            '>' => match tokens.last().cloned() {
                Some(Token::Cmp(Comparison::Gt)) => {
                    if let Some(tok) = tokens.last_mut() {
                        *tok = Token::Op(Operator::Shr);
                    }
                }
                _ => tokens.push(Token::Cmp(Comparison::Gt)),
            },
            '<' => match tokens.last().cloned() {
                Some(Token::Cmp(Comparison::Lt)) => {
                    if let Some(tok) = tokens.last_mut() {
                        *tok = Token::Op(Operator::Shl);
                    }
                }
                _ => tokens.push(Token::Cmp(Comparison::Lt)),
            },
            '=' => match tokens.last().cloned() {
                Some(Token::Sign(Sign::Equal)) => {
                    if let Some(tok) = tokens.last_mut() {
                        *tok = Token::Cmp(Comparison::Eq);
                    }
                }
                Some(Token::Cmp(Comparison::Lt)) => {
                    if let Some(tok) = tokens.last_mut() {
                        *tok = Token::Cmp(Comparison::Le);
                    }
                }
                Some(Token::Cmp(Comparison::Gt)) => {
                    if let Some(tok) = tokens.last_mut() {
                        *tok = Token::Cmp(Comparison::Ge);
                    }
                }
                _ => tokens.push(Token::Sign(Sign::Equal)),
            },
            other => return Err(Error::BadToken(other)),
        }
    }
    tokens.push(Token::EndTok);

    if parens.len() > 0 {
        return Err(Error::MismatchParams);
    }

    Ok(tokens.into())
}
