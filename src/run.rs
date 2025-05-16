use crate::scanner;

pub fn run(script: String) {
    let mut scanner = scanner::Scanner::new(script);
    let tokens = scanner.scan_tokens();

    tokens.iter().for_each(|t| println!("{:?}", t));
}
