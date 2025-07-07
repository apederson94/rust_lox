use std::sync::atomic::{AtomicBool, Ordering};

use crate::interpretable::RuntimeError;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn had_error() -> bool {
    let value = HAD_ERROR.load(Ordering::Relaxed);
    value
}

pub fn error(line: u32, msg: String) {
    println!("[line {}] Error: {}", line, msg);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn runtime_error(err: &RuntimeError) {
    eprintln!("{}", err);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}
