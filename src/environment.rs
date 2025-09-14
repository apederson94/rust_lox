use std::{any::Any, collections::HashMap};

use crate::{errors, token::Token};

#[derive(Clone)]
pub enum EnvironmentValue {
    Number(f64),
    Str(String),
    Boolean(bool),
}

pub enum EnvironmentError {
    UndefinedVariable,
}

pub struct Environment {
    values: HashMap<String, EnvironmentValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: EnvironmentValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<EnvironmentValue, EnvironmentError> {
        if let Some(v) = self.values.get(name.lexeme()) {
            return Ok(v.clone());
        }

        let error_msg = format!("Undefined variable {}", name.lexeme());
        errors::error(name.line(), error_msg);
        return Err(EnvironmentError::UndefinedVariable);
    }
}
