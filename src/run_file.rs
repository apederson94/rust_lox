use crate::{errors, run};
use std::{fs, process};

pub fn run_file(path: String) {
    let data = fs::read_to_string(path);

    match data {
        Ok(script) => run::run(script),
        Err(e) => {
            panic!("Failed to run script file: {:?}", e)
        }
    }
    if errors::had_error() {
        process::exit(65)
    }
}
