use crate::expr::Expr;
use crate::token::TokenType;

pub trait AstPrinter {
    fn print(&self) -> String;
}

impl AstPrinter for Expr {
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
                _ => format!("Error: Unknown literal type!"),
            },
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme(), right.print()),
        }
    }
}
