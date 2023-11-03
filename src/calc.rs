// Copyright (c) 2023 Ethan Uppal. All rights reserved.

use std::{error::Error, fmt};

/// A binary operation.
#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Evalutates the operator on the given inputs.
    fn eval(&self, lhs: f64, rhs: f64) -> f64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }

    fn parse(token: &str) -> Option<Op> {
        match token {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            _ => None,
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        String::from(match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        })
    }
}

#[derive(Debug)]
pub struct ExprParseError<'a> {
    /// The invalid expression we tried to parse.
    source: &'a str,

    /// The number of tokens left on the stack.
    remaining: usize,
}

impl<'a> Error for ExprParseError<'a> {}

impl<'a> fmt::Display for ExprParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing failed for expression '{}': {} token(s) left on the stack.", self.source, self.remaining)
    }
}

impl<'a> ExprParseError<'a> {
    fn from(source: &str, stack: Vec<Expr>) -> ExprParseError {
        ExprParseError {
            source: source,
            remaining: stack.len(),
        }
    }
}

/// An expression tree.
#[derive(Clone)]
pub enum Expr {
    Num(f64),
    Bin(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    pub fn infix_string(&self) -> String {
        match self {
            Expr::Num(value) => value.to_string(),
            Expr::Bin(lhs, op, rhs) => format!(
                "({} {} {})",
                lhs.infix_string(),
                op.to_string(),
                rhs.infix_string()
            ),
        }
    }

    /// Evaluates the expression.
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(value) => *value,
            Expr::Bin(lhs, op, rhs) => op.eval(lhs.eval(), rhs.eval()),
        }
    }

    /// Parses the expression `source` and returns an expression tree.
    pub fn parse(source: &str) -> Result<Expr, ExprParseError> {
        // Token array
        let tokens: Vec<&str> = source.split_whitespace().collect();

        // The expression stack
        let mut stack: Vec<Expr> = vec![];

        // Build the final expression
        for i in 0..tokens.len() {
            let token = tokens[i];
            if let Some(num) = token.parse::<f64>().ok() {
                stack.push(Expr::Num(num));
            } else if let Some(op) = Op::parse(token) {
                if stack.len() < 2 {
                    return Err(ExprParseError::from(source, stack));
                }
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(Expr::Bin(Box::new(lhs), op, Box::new(rhs)));
            } else {
                return Err(ExprParseError::from(source, stack));
            }
        }

        // Return the final expression
        match stack.len() {
            1 => Ok(stack[0].clone()),
            _ => Err(ExprParseError::from(source, stack)),
        }
    }
}
