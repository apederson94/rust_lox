use errors::RuntimeError;

use crate::{errors, lox_value::LoxValue};

pub trait Interpretable {
    fn interpret(&self) -> Result<LoxValue, RuntimeError>;
}
