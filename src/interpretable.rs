use crate::{expr::Expr, lox_value::LoxValue, token::TokenType};

pub trait Interpretable {
    fn interpret(&self) -> LoxValue;
}

impl Interpretable for Expr {
    fn interpret(&self) -> LoxValue {
        match self {
            Expr::Literal { value: tt } => match tt {
                TokenType::Number(n) => LoxValue::Number(*n),
                TokenType::Str(s) => LoxValue::Str(s.clone()),
                TokenType::True => LoxValue::Bool(true),
                TokenType::False => LoxValue::Bool(false),
                TokenType::EndOfFile => LoxValue::Nil,
                _ => LoxValue::Nil,
            },
            _ => LoxValue::Nil,
        }
    }
}
