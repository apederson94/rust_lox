use std::fmt::{format, Display};

use crate::{
    errors,
    expr::Expr,
    lox_value::LoxValue,
    token::{Token, TokenType},
};

pub trait Interpretable {
    fn interpret(&self) -> Result<LoxValue, RuntimeError>;
}

fn evaluate(expr: &Expr) -> Result<LoxValue, RuntimeError> {
    match expr {
        Expr::Literal { value } => match value.type_info() {
            TokenType::Number(n) => Ok(LoxValue::Number(*n)),
            TokenType::Str(s) => Ok(LoxValue::Str(s.clone())),
            TokenType::True => Ok(LoxValue::Bool(true)),
            TokenType::False => Ok(LoxValue::Bool(false)),
            TokenType::EndOfFile => Ok(LoxValue::Nil),
            _ => Err(RuntimeError::new(
                value.line(),
                String::from(value.lexeme()),
                RuntimeErrorType::InvalidLiteral,
            )),
        },
        Expr::Grouping { expression } => expression.interpret(),
        Expr::Unary { operator, right } => {
            let right_value = right.interpret()?;
            match operator.type_info() {
                TokenType::Minus => {
                    if let LoxValue::Number(n) = right_value {
                        Ok(LoxValue::Number(-n))
                    } else {
                        Err(RuntimeError::new(
                            operator.line(),
                            String::from(operator.lexeme()),
                            RuntimeErrorType::OperandMustBeNumber,
                        ))
                    }
                }

                // handle falsiness (false, nil are both falsey, everything else is truthy)
                // default case is false because we are doing a logical not here
                TokenType::Bang => match right_value {
                    LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
                    LoxValue::Nil => Ok(LoxValue::Bool(true)),
                    _ => Ok(LoxValue::Bool(false)),
                },
                _ => Err(RuntimeError::new(
                    operator.line(),
                    String::from(operator.lexeme()),
                    RuntimeErrorType::InvalidUnaryOperator,
                )),
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
                TokenType::Minus => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 - n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::Plus => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 + n2)),
                    (LoxValue::Str(s1), LoxValue::Str(s2)) => {
                        Ok(LoxValue::Str(format!("{}{}", s1, s2)))
                    }
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumberOrString,
                    )),
                },

                TokenType::Slash => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 / n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::Star => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 * n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::Greater => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 > n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::GreaterEqual => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 >= n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::Less => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 < n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::LessEqual => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 <= n2)),
                    _ => Err(RuntimeError::new(
                        operator.line(),
                        String::from(operator.lexeme()),
                        RuntimeErrorType::OperandMustBeNumber,
                    )),
                },

                TokenType::EqualEqual => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 == n2)),
                    (LoxValue::Str(s1), LoxValue::Str(s2)) => Ok(LoxValue::Bool(s1 == s2)),
                    (LoxValue::Bool(b1), LoxValue::Bool(b2)) => Ok(LoxValue::Bool(b1 == b2)),
                    _ => Ok(LoxValue::Bool(false)),
                },

                TokenType::BangEqual => match (left_value, right_value) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 != n2)),
                    (LoxValue::Str(s1), LoxValue::Str(s2)) => Ok(LoxValue::Bool(s1 != s2)),
                    (LoxValue::Bool(b1), LoxValue::Bool(b2)) => Ok(LoxValue::Bool(b1 != b2)),
                    (LoxValue::Nil, LoxValue::Nil) => Ok(LoxValue::Bool(true)),
                    _ => Ok(LoxValue::Bool(true)),
                },

                _ => Err(RuntimeError::new(
                    operator.line(),
                    String::from(operator.lexeme()),
                    RuntimeErrorType::InvalidBinaryOperator,
                )),
            }
        }
        Expr::Conditional {
            condition,
            consequent,
            alternative,
        } => {
            panic!("Conditional expression not implemented")
        }
    }
}

impl Interpretable for Expr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        match evaluate(self) {
            Ok(value) => Ok(value),
            Err(err) => {
                errors::error(0, format!("{}", err));
                Err(err)
            }
        }
    }
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
    InvalidExpression,
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
            RuntimeErrorType::InvalidExpression => {
                write!(
                    f,
                    "[line {}]: Invalid expression: {:?}",
                    self.line, self.cause
                )
            }
        }
    }
}
