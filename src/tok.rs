// use std::ops::*;

use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

use crate::value::{NumberType, Ty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comparison {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    // add `+`
    Add,
    // `-`
    Sub,
    // '%'
    Rem,
    // `*`
    Mul,
    // `/`
    Div,
    // `>>`
    Shr,
    // `<<`
    Shl,
    // `|`
    BitOr,
    // `&`
    BitAnd,
    // `^`
    BitXor,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Neq,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    StartTok,
    Num(NumberType),
    Op(Operator),
    Bracket(char),
    Cmp(Comparison),
    Sign(Sign),
    EndTok,
}

#[derive(Debug, Clone)]
pub struct Expresions {
    pub queue: Vec<Token>,
}
macro_rules! close_expr_bracket {
    ($open:literal .. $close:literal, $stack:ident, $queue:ident) => {{
        while !$stack.is_empty() && $stack[$stack.len() - 1] != Token::Bracket($open) {
            $queue.push($stack.pop().unwrap());
        }
        $stack.push(Token::Bracket($close));
    }};
}

impl Expresions {
    pub fn eval(&mut self) -> Option<NumberType> {
        self.queue.reverse();
        let mut stack = Vec::with_capacity(self.queue.len());
        while let Some(token) = self.queue.pop() {
            match token {
                Token::Num(num) => stack.push(num),
                Token::Op(op) => {
                    let Some(right) = stack.pop() else {
                        continue;
                    };
                    let Some(left) = stack.pop() else {
                        continue;
                    };
                    match op {
                        Operator::Add => stack.push((left + right).into()),
                        Operator::Sub => stack.push((left - right).into()),
                        Operator::Rem => stack.push((left % right).into()),
                        Operator::Mul => stack.push((left * right).into()),
                        Operator::Div => stack.push((left / right).into()),
                        Operator::Shr => stack.push(left.shr(right)),
                        Operator::Shl => stack.push(left.shl(right)),
                        Operator::BitOr => stack.push(left.bitor(right)),
                        Operator::BitAnd => stack.push(left.bitand(right)),
                        Operator::BitXor => stack.push(left.bitxor(right)),
                    }
                }
                Token::Cmp(cmp) => {
                    let Some(right) = stack.pop() else {
                        continue;
                    };
                    let Some(left) = stack.pop() else {
                        continue;
                    };
                    let val = match cmp {
                        Comparison::Eq => left == right,
                        Comparison::Ne => left != right,
                        Comparison::Le => left <= right,
                        Comparison::Lt => left < right,
                        Comparison::Ge => left >= right,
                        Comparison::Gt => left > right,
                    };
                    let val = NumberType::new(Ty::BOOL, if val { 1 } else { 0 });
                    stack.push(val);
                }
                _ => {}
            }
        }
        stack.pop()
    }

    fn from_parser(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        let mut queue = Vec::with_capacity(tokens.len());
        let mut stack = Vec::with_capacity(tokens.len());

        while let Some(token) = tokens.pop() {
            match token {
                Token::Num(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Cmp(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token)
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => close_expr_bracket!('('..')', stack, queue),
                Token::Bracket('{') => stack.push(token),
                Token::Bracket('}') => close_expr_bracket!('{'..'}', stack, queue),
                Token::Bracket('[') => stack.push(token),
                Token::Bracket(']') => close_expr_bracket!('['..']', stack, queue),

                _ => {}
            }
        }
        while let Some(token) = stack.pop() {
            queue.push(token);
        }

        Self { queue }
    }
}

impl From<Vec<Token>> for Expresions {
    fn from(v: Vec<Token>) -> Self {
        Self::from_parser(v)
    }
}
