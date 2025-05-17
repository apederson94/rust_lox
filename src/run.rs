use std::process;

use crate::{errors, scanner};

pub fn run(script: String) {
    let mut scanner = scanner::Scanner::new(script);
    let tokens = scanner.scan_tokens();

    if errors::had_error() {
        process::exit(65);
    }

    tokens.iter().for_each(|t| println!("{:?}", t));
}
