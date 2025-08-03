use crate::{errors::RuntimeError, expr::Expr, interpretable::Interpretable, lox_value::LoxValue};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl Interpretable for Stmt {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        match self {
            Stmt::Expression(expr) => match expr.interpret() {
                Ok(_) => Ok(LoxValue::Nil),
                Err(error) => Err(error),
            },
            Stmt::Print(expr) => match expr.interpret() {
                Ok(value) => {
                    println!("{}", value);
                    Ok(LoxValue::Nil)
                }
                Err(error) => Err(error),
            },
        }
    }
}
