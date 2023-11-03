// Copyright (c) 2023 Ethan Uppal. All rights reserved.

pub mod io {
    /// Prints the help message for the calculator app.
    #[rustfmt::skip]
    pub fn print_help(name: &str) {
        println!("A calculator app in rust.");
        println!();
        println!("usage: {}                  Begins the calculator app", name);
        println!("   or: {} -h|--help        Prints this help", name);
        println!("   or: {} -v|--version     Prints versioning information", name);
    }

    /// Prints the versioning information for the calculator app.
    #[rustfmt::skip]
    pub fn print_version() {
        println!("A calculator app in rust. Copyright (c) 2023 Ethan Uppal. All rights reserved.");
    }
}

mod calc {
    use std::{error::Error, fmt};

    /// A binary operation.
    pub enum Op {
        Add,
        Sub,
    }

    #[derive(Debug)]
    pub struct ExprParseError<'a> {
        /// The invalid expression we tried to parse.
        source: &'a str,

        /// The number of tokens left on the stack.
        remaining: i32,
    }

    impl<'a> Error for ExprParseError<'a> {}

    impl<'a> fmt::Display for ExprParseError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parsing failed for expression '{}' ({} token(s) left on the stack)., ", self.source, self.remaining)
        }
    }

    /// An expression tree.
    pub enum Expr {
        Num(f64),
        Bin(Box<Expr>, Op, Box<Expr>),
    }

    /// Parses the expression `source` and returns an expression tree.
    pub fn parse(source: &str) -> Result<Expr, ExprParseError> {
        Err(ExprParseError {
            source: "test",
            remaining: 0,
        })
    }
}
