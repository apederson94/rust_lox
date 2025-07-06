use std::fmt::Display;

use crate::{
    expr::Expr,
    lox_value::LoxValue,
    token::{Token, TokenType},
};

pub trait Interpretable {
    fn interpret(&self) -> Result<LoxValue, InterpretationError>;
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
                _ => Err(InterpretationError::InvalidLiteral(value.clone())),
            },
            Expr::Grouping { expression } => expression.interpret(),
            Expr::Unary { operator, right } => {
                let right_value = right.interpret()?;
                match operator.type_info() {
                    TokenType::Minus => {
                        if let LoxValue::Number(n) = right_value {
                            Ok(LoxValue::Number(-n))
                        } else {
                            Err(InterpretationError::OperandMustBeNumber(operator.clone()))
                        }
                    }

                    // handle falsiness (false, nil are both falsey, everything else is truthy)
                    // default case is false because we are doing a logical not here
                    TokenType::Bang => match right_value {
                        LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
                        LoxValue::Nil => Ok(LoxValue::Bool(true)),
                        _ => Ok(LoxValue::Bool(false)),
                    },
                    op => Err(InterpretationError::InvalidUnaryOperator(op.clone())),
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
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 - n2))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::Plus => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 + n2))
                        }
                        (LoxValue::Str(s1), LoxValue::Str(s2)) => {
                            Ok(LoxValue::Str(format!("{}{}", s1, s2)))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumberOrString(
                            operator.clone(),
                        )),
                    },

                    TokenType::Slash => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 / n2))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::Star => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 * n2))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::Greater => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 > n2)),
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::GreaterEqual => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Bool(n1 >= n2))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::Less => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Bool(n1 < n2)),
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

                    TokenType::LessEqual => match (left_value, right_value) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Bool(n1 <= n2))
                        }
                        _ => Err(InterpretationError::OperandMustBeNumber(operator.clone())),
                    },

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

                    tt => Err(InterpretationError::InvalidBinaryOperator(tt.clone())),
                }
            }
            e => Err(InterpretationError::InvalidExpression(e.clone())),
        }
    }
}

pub enum InterpretationError {
    OperandMustBeNumber(Token),
    OperandMustBeNumberOrString(Token),
    InvalidLiteral(TokenType),
    InvalidUnaryOperator(TokenType),
    InvalidBinaryOperator(TokenType),
    InvalidExpression(Expr),
}

impl Display for InterpretationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretationError::OperandMustBeNumber(token) => {
                write!(f, "Operand must be a number: {:?}", token)
            }
            InterpretationError::OperandMustBeNumberOrString(token) => {
                write!(f, "Operand must be a number or string: {:?}", token)
            }
            InterpretationError::InvalidLiteral(token_type) => {
                write!(f, "Invalid literal: {:?}", token_type)
            }
            InterpretationError::InvalidUnaryOperator(token_type) => {
                write!(f, "Invalid unary operator: {:?}", token_type)
            }
            InterpretationError::InvalidBinaryOperator(token_type) => {
                write!(f, "Invalid binary operator: {:?}", token_type)
            }
            InterpretationError::InvalidExpression(expr) => {
                write!(f, "Invalid expression: {:?}", expr)
            }
        }
    }
}

// pub struct InterpretationError {
//     message: String,
// }

// impl InterpretationError {
//     fn new(expr: &Expr) -> Self {
//         InterpretationError {
//             message: format!("Unexpected token {:?}", expr),
//         }
//     }

//     fn message(&self) -> &str {
//         &self.message
//     }
// }
