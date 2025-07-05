use crate::expr::Expr;
use crate::token::TokenType;

pub trait ASTPrintable {
    fn print(&self) -> String;
}

impl ASTPrintable for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!("({} {} {})", operator.lexeme(), left.print(), right.print())
            }
            Expr::Grouping { expression } => format!("(group {})", expression.print()),
            Expr::Literal { value } => match value {
                TokenType::Number(num) => format!("{}", num),
                TokenType::Str(text) => format!("\"{}\"", text),
                TokenType::Identifier(ident) => format!("{}", ident),
                TokenType::EndOfFile => String::from("EndOfFile"),
                _ => format!("Error: Unknown literal type!"),
            },
            Expr::Conditional {
                condition,
                consequent,
                alternative,
            } => {
                format!(
                    "(if {} {} else {})",
                    condition.print(),
                    consequent.print(),
                    alternative.print()
                )
            }
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme(), right.print()),
        }
    }
}
