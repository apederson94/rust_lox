use std::env::args;

mod errors;
mod run;
mod run_file;
mod run_prompt;
mod scanner;
mod token;

fn main() {
    println!("Hello, rust_lox!");
    let arguments: Vec<String> = args().collect();

    if arguments.len() > 2 {
        std::process::exit(64);
    } else if arguments.len() == 2 {
        run_file::run_file(arguments[1].clone())
    } else {
        run_prompt::run_prompt()
    }
}
