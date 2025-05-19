use crate::run;
use std::fs;

pub fn run_file(path: String) {
    let data = fs::read_to_string(path);

    match data {
        Ok(script) => run::run(script),
        Err(e) => {
            panic!("Failed to run script file: {:?}", e)
        }
    }
}
