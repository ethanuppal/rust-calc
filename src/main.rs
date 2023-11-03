// Copyright (c) 2023 Ethan Uppal. All rights reserved.

use std::io::{self, BufRead, Write};
mod app;
mod calc;

fn main() {
    app::from_args().run(|app| {
        let mut lines = io::stdin().lock().lines();

        loop {
            // Print input indicator
            print!("> ");
            io::stdout().flush().unwrap();

            // Read line
            if let Some(line) = lines.next() {
                // Process line
                match line {
                    Ok(result) => match calc::Expr::parse(&result) {
                        Ok(expr) => {
                            println!(
                                "{} = {}",
                                expr.infix_string(),
                                expr.eval()
                            );
                        }
                        Err(err) => app.print_error("parse", &err.to_string()),
                    },
                    Err(err) => app.print_error("io", &err.to_string()),
                }
            } else {
                break;
            }
        }
    });
}
