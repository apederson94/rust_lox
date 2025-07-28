use std::{
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn had_error() -> bool {
    let value = HAD_ERROR.load(Ordering::Relaxed);
    value
}

pub fn had_runtime_error() -> bool {
    let value = HAD_RUNTIME_ERROR.load(Ordering::Relaxed);
    value
}

pub fn error(line: u32, msg: String) {
    eprintln!("HERE!");
    eprintln!("[line {}] Error: {}", line, msg);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn runtime_error(err: &RuntimeError) {
    eprintln!("{}", err);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}

pub struct RuntimeError {
    line: u32,
    cause: String,
    error_type: RuntimeErrorType,
}

impl RuntimeError {
    pub fn new(line: u32, cause: String, error_type: RuntimeErrorType) -> Self {
        Self {
            line,
            cause,
            error_type,
        }
    }
}

pub enum RuntimeErrorType {
    OperandMustBeNumber,
    OperandMustBeNumberOrString,
    InvalidLiteral,
    InvalidUnaryOperator,
    InvalidBinaryOperator,
    DivideByZero,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            RuntimeErrorType::OperandMustBeNumber => {
                write!(
                    f,
                    "[line {}]: Operand must be a number: {}",
                    self.line, self.cause
                )
            }
            RuntimeErrorType::OperandMustBeNumberOrString => {
                write!(
                    f,
                    "[line {}]: Operands must both be either numbers or strings: {}",
                    self.line, self.cause
                )
            }
            RuntimeErrorType::InvalidLiteral => {
                write!(f, "[line {}]: Invalid literal: {:?}", self.line, self.cause)
            }
            RuntimeErrorType::InvalidUnaryOperator => {
                write!(
                    f,
                    "[line {}]: Invalid unary operator: {:?}",
                    self.line, self.cause
                )
            }
            RuntimeErrorType::InvalidBinaryOperator => {
                write!(
                    f,
                    "[line {}]: Invalid binary operator: {:?}",
                    self.line, self.cause
                )
            }
            RuntimeErrorType::DivideByZero => {
                write!(
                    f,
                    "[line {}]: Cannot divide by zero: {:?}",
                    self.line, self.cause
                )
            }
        }
    }
}
