// Copyright (c) 2023 Ethan Uppal. All rights reserved.

mod app;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();
    app::io::print_help(arg[0].as_str());
}
