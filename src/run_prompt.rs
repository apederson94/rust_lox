use crate::run;
use std::io::{self, Write};

pub fn run_prompt() {
    println!("Starting REPL...");
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        print!("> ");
        io::stdout().flush();
        match stdin.read_line(input) {
            Ok(_) => run::run(input.clone()),
            Err(_) => break,
        }
        input.clear();
    }
}
