use std::env::args;

mod ast_printable;
mod environment;
mod errors;
mod expr;
mod interpretable;
mod lox_value;
mod parser;
mod runner;
mod scanner;
mod stmt;
mod token;

fn main() {
    println!("Hello, rust_lox!");
    let arguments: Vec<String> = args().collect();

    if arguments.len() > 2 {
        std::process::exit(64);
    } else if arguments.len() == 2 {
        if let Err(e) = runner::run_file(arguments[1].clone()) {
            eprintln!("{}", e);
            std::process::exit(99);
        }
    } else {
        runner::run_prompt()
    }
}
