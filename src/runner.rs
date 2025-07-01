use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    process,
};

use crate::{ast_printer::AstPrinter, errors, parser, scanner};

pub fn run_file(path: String) -> Result<(), RunnerError> {
    let data = fs::read_to_string(path);

    match data {
        Ok(script) => Ok(run(script)),
        Err(_) => Err(RunnerError::FailedToRunFile),
    }
}

pub fn run_prompt() {
    println!("Starting REPL...");
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        match stdin.read_line(input) {
            Ok(_) => run(input.clone()),
            Err(_) => break,
        }
        input.clear();
    }
}

fn run(script: String) {
    let mut scanner = scanner::Scanner::new(script);
    let tokens = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens.clone());
    let expression = parser.parse();

    if errors::had_error() {
        process::exit(65);
    }

    match expression {
        Some(expr) => println!("{}", expr.print()),
        None => (),
    }

    tokens.iter().for_each(|t| println!("{:?}", t));
}

pub enum RunnerError {
    FailedToRunFile,
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToRunFile => write!(f, "Failed to run file!"),
        }
    }
}
