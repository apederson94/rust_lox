use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenType,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
