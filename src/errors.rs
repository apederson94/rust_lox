use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn had_error() -> bool {
    let value = HAD_ERROR.load(Ordering::Relaxed);
    value
}

pub fn error(line: u32, msg: String) {
    report(line, String::from(""), msg)
}

fn report(line: u32, location: String, msg: String) {
    println!("[line {}] Error {}: {}", line, location, msg);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
