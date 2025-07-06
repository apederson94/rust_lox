use std::boxed;

use crate::{expr::Expr, lox_value::LoxValue, token::TokenType};

pub trait Interpretable {
    fn interpret(&self) -> Result<LoxValue, InterpretationError>;
}

impl Expr {
    fn get_number(&self, value: &LoxValue) -> Result<f64, InterpretationError> {
        match value {
            LoxValue::Number(n) => Ok(*n),
            _ => Err(InterpretationError::new(self)),
        }
    }

    fn get_str(&self, value: &LoxValue) -> Result<String, InterpretationError> {
        match value {
            LoxValue::Str(s) => Ok(s.clone()),
            _ => Err(InterpretationError::new(self)),
        }
    }

    fn get_bool(&self, value: &LoxValue) -> Result<bool, InterpretationError> {
        match value {
            LoxValue::Bool(b) => Ok(*b),
            _ => Err(InterpretationError::new(self)),
        }
    }
}

impl Interpretable for Expr {
    fn interpret(&self) -> Result<LoxValue, InterpretationError> {
        match self {
            Expr::Literal { value } => match value {
                TokenType::Number(n) => Ok(LoxValue::Number(*n)),
                TokenType::Str(s) => Ok(LoxValue::Str(s.clone())),
                TokenType::True => Ok(LoxValue::Bool(true)),
                TokenType::False => Ok(LoxValue::Bool(false)),
                TokenType::EndOfFile => Ok(LoxValue::Nil),
                _ => Err(InterpretationError::new(self)),
            },
            Expr::Grouping { expression } => expression.interpret(),
            Expr::Unary { operator, right } => {
                let right_value = right.interpret()?;
                match operator.type_info() {
                    TokenType::Minus => {
                        if let LoxValue::Number(n) = right_value {
                            Ok(LoxValue::Number(-n))
                        } else {
                            Err(InterpretationError::new(self))
                        }
                    }

                    // handle falsiness (false, nil are both falsey, everything else is truthy)
                    // default case is false because we are doing a logical not here
                    TokenType::Bang => match right_value {
                        LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
                        LoxValue::Nil => Ok(LoxValue::Bool(true)),
                        _ => Ok(LoxValue::Bool(false)),
                    },
                    _ => Err(InterpretationError::new(self)),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = left.interpret()?;
                let right_value = right.interpret()?;

                match operator.type_info() {
                    TokenType::Minus => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Number(n1 - n2))
                    }

                    TokenType::Plus => {
                        // Addition of numbers
                        if let Ok(n1) = self.get_number(&left_value) {
                            match right_value {
                                LoxValue::Number(n2) => Ok(LoxValue::Number(n1 + n2)),
                                _ => Err(InterpretationError::new(self)),
                            }

                        // String concat
                        } else {
                            if let Ok(s1) = self.get_str(&left_value) {
                                match right_value {
                                    LoxValue::Str(s2) => Ok(LoxValue::Str(format!("{}{}", s1, s2))),
                                    _ => Err(InterpretationError::new(self)),
                                }
                            } else {
                                Err(InterpretationError::new(self))
                            }
                        }
                    }

                    TokenType::Slash => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Number(n1 / n2))
                    }

                    TokenType::Star => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Number(n1 * n2))
                    }

                    TokenType::Greater => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Bool(n1 > n2))
                    }

                    TokenType::GreaterEqual => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Bool(n1 >= n2))
                    }

                    TokenType::Less => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Bool(n1 < n2))
                    }

                    TokenType::LessEqual => {
                        let n1 = self.get_number(&left_value)?;
                        let n2 = self.get_number(&right_value)?;
                        Ok(LoxValue::Bool(n1 <= n2))
                    }

                    TokenType::EqualEqual => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Bool(n1 == n2))
                        }
                        (LoxValue::Str(s1), LoxValue::Str(s2)) => Ok(LoxValue::Bool(s1 == s2)),
                        (LoxValue::Bool(b1), LoxValue::Bool(b2)) => Ok(LoxValue::Bool(b1 == b2)),
                        _ => Ok(LoxValue::Bool(false)),
                    },

                    TokenType::BangEqual => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Bool(n1 != n2))
                        }
                        (LoxValue::Str(s1), LoxValue::Str(s2)) => Ok(LoxValue::Bool(s1 != s2)),
                        (LoxValue::Bool(b1), LoxValue::Bool(b2)) => Ok(LoxValue::Bool(b1 != b2)),
                        (LoxValue::Nil, LoxValue::Nil) => Ok(LoxValue::Bool(true)),
                        _ => Ok(LoxValue::Bool(true)),
                    },

                    _ => Err(InterpretationError::new(self)),
                }
            }
            _ => Err(InterpretationError::new(self)),
        }
    }
}

pub struct InterpretationError {
    message: String,
}

impl InterpretationError {
    fn new(expr: &Expr) -> Self {
        InterpretationError {
            message: format!("Unexpected token {:?}", expr),
        }
    }

    fn message(&self) -> &str {
        &self.message
    }
}
