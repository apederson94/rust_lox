use crate::{errors::RuntimeError, expr::Expr, interpretable::Interpretable, lox_value::LoxValue};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var { name: String, initializer: Expr },
}

impl Interpretable for Stmt {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        match self {
            Stmt::Expression(expr) => expr.interpret(),
            Stmt::Print(expr) => {
                let value = expr.interpret()?;
                println!("{}", value);
                Ok(value)
            }
            Stmt::Var {
                name,
                initializer: expr,
            } => expr.interpret(),
        }
    }
}
