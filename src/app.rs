// Copyright (c) 2023 Ethan Uppal. All rights reserved.

/// The calculator app.
/// Create with
/// ```
/// mod app;
/// let app = app::from_args();
/// ```
pub struct App {
    /// The program arguments. Invariant: non-empty.
    args: Vec<String>,
}

/// Creates a new app from the program arguments.
pub fn from_args() -> App {
    use std::env;

    let args: Vec<String> = env::args().collect();
    assert!(!args.is_empty());
    App { args: args }
}

pub fn from_given_args(args: Vec<String>) -> App {
    assert!(!args.is_empty());
    App { args: args }
}

impl App {
    /// Returns the executable name of the app.
    pub fn name(&self) -> &str {
        // The first argument should be the program that was invoked from the
        // shell. This is true in most cases.
        return self.args[0].as_str();
    }

    /// Runs the given code or prints help and versioning info, depending on the
    /// arguments passed.
    pub fn run(&self, main: fn(&App) -> ()) {
        if self.args.len() == 1 {
            main(self);
        } else {
            match self.args.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
                [1..]
            {
                ["-v"] | ["--version"] => {
                    self.print_description();
                    println!();
                    self.print_version()
                }
                ["-h"] | ["--help"] => {
                    self.print_description();
                    println!();
                    self.print_help()
                }
                _ => self.print_arg_error("Unknown argument."),
            }
        }
    }

    /// Prints a short description of the app.
    pub fn print_description(&self) {
        println!("A calculator app in rust.");
    }

    /// Prints the help message for the app.
    #[rustfmt::skip]
    pub fn print_help(&self) {
        println!("usage: {}                  Begins the calculator app", self.name());
        println!("   or: {} -h|--help        Prints this help", self.name());
        println!("   or: {} -v|--version     Prints versioning information", self.name());
    }

    /// Prints the versioning information for the app.
    #[rustfmt::skip]
    pub fn print_version(&self) {
        println!("Copyright (c) 2023 Ethan Uppal. All rights reserved.");
    }

    /// Prints an error message with the given under a given context.
    ///
    /// Example:
    /// ```
    /// app.print_error("parse", "Mismatched types.");
    /// ```
    pub fn print_error(&self, ctx: &str, msg: &str) {
        eprintln!("{} error: {}", ctx, msg);
    }

    /// Logs an argument-parsing error to the console.
    pub fn print_arg_error(&self, msg: &str) {
        self.print_error("arg", msg);
        eprintln!("(invocation: '{}')", self.args.join(" "));
        eprintln!();
        self.print_help();
    }
}

#[cfg(test)]
mod test {
    use crate::app;

    #[test]
    #[should_panic]
    fn empty_args() {
        let _app = app::from_given_args(vec![]);
    }
}
