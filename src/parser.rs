use crate::{
    errors,
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
};

struct ParseError {
    message: String,
    line: u32,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.append(self.statement());
        }
        // old code
        // match self.expression_list() {
        //     Ok(expr) => Some(expr),
        //     Err(err) => {
        //         errors::error(err.line, err.message);
        //         None
        //     }
        // }
        return Some(statements);
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value");
        return match value {
            Ok(e) => Ok(Stmt::Print(e)),
            Err(e) => Err(e),
        };
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression");
        return match expression {
            Ok(e) => Ok(Stmt::Expression(e)),
            Err(e) => Err(e),
        };
    }

    fn expression_list(&mut self) -> Result<Expr, ParseError> {
        return self.parse_binary(|s| s.expression(), &[TokenType::Comma]);
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.conditional();
    }

    fn parse_binary(
        &mut self,
        mut nxt: impl FnMut(&mut Self) -> Result<Expr, ParseError>,
        tokens: &[TokenType],
    ) -> Result<Expr, ParseError> {
        let mut expr = nxt(self)?;

        while self.match_tokens(tokens) {
            let operator = self.previous().clone();
            let right = nxt(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn conditional(&mut self) -> Result<Expr, ParseError> {
        let equality_expr = self.equality();

        if self.match_tokens(&[TokenType::QuestionMark]) {
            let consequent = self.expression()?;

            if self.match_tokens(&[TokenType::Colon]) {
                let alternative = self.expression()?;

                return Ok(Expr::Conditional {
                    condition: Box::new(equality_expr?),
                    consequent: Box::new(consequent),
                    alternative: Box::new(alternative),
                });
            }
        }

        equality_expr
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary(
            |s| s.comparison(),
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary(
            |s| s.term(),
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary(|s| s.factor(), &[TokenType::Minus, TokenType::Plus])
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary(|s| s.unary(), &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().type_info() {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                let operator = self.previous().clone();
                let right = self.unary()?;
                Ok(Expr::Unary {
                    operator: operator,
                    right: Box::new(right),
                })
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().type_info() {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal {
                    value: self.previous().clone(),
                })
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal {
                    value: self.previous().clone(),
                })
            }
            TokenType::Number(_) => {
                self.advance();
                Ok(Expr::Literal {
                    value: self.previous().clone(),
                })
            }

            TokenType::Str(_) => {
                self.advance();
                Ok(Expr::Literal {
                    value: self.previous().clone(),
                })
            }

            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }

            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal {
                    value: self.previous().clone(),
                })
            }

            TokenType::EndOfFile => Ok(Expr::Literal {
                value: self.peek().clone(),
            }),

            _ => Err(ParseError {
                message: format!("Unexpected token: {}.", self.tokens[self.current]),
                line: self.peek().line(),
            }),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                message: format!("Unexpected token: {}.", message),
                line: self.previous().line(),
            })
        }
    }

    fn match_tokens(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().type_info() == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        if self.current == 0 {
            &self.tokens[0]
        } else {
            &self.tokens[self.current - 1]
        }
    }

    fn is_at_end(&self) -> bool {
        *self.peek().type_info() == TokenType::EndOfFile
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if *self.previous().type_info() == TokenType::Semicolon {
                return;
            }

            match self.peek().type_info() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}
