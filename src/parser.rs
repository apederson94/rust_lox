use crate::{
    errors,
    expr::Expr,
    token::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn parse_binary(
        &mut self,
        mut nxt: impl FnMut(&mut Self) -> Expr,
        tokens: &[TokenType],
    ) -> Expr {
        let mut expr = nxt(self);

        while self.match_tokens(tokens) {
            let operator = self.advance().clone();
            let right = nxt(self);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        self.parse_binary(
            |s| s.comparison(),
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Expr {
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

    fn term(&mut self) -> Expr {
        self.parse_binary(|s| s.factor(), &[TokenType::Minus, TokenType::Plus])
    }

    fn factor(&mut self) -> Expr {
        self.parse_binary(|s| s.unary(), &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Expr {
        if self.is_at_end() {
            errors::error(self.previous().line(), String::from("Unexpected token"));
        };

        match self.peek().type_info() {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                let operator = self.previous().clone();
                let right = self.unary();
                Expr::Unary {
                    operator: operator,
                    right: Box::new(right),
                }
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        if self.is_at_end() {
            errors::error(self.previous().line(), String::from("Unexpected token"));
        }

        match self.peek().type_info() {
            TokenType::False => {
                self.advance();
                Expr::Literal {
                    value: TokenType::False,
                }
            }
            TokenType::True => {
                self.advance();
                Expr::Literal {
                    value: TokenType::True,
                }
            }
            TokenType::Number(n) => {
                let num = n.clone();
                self.advance();
                Expr::Literal {
                    value: TokenType::Number(num),
                }
            }

            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => errors::error(self.previous().line(), "Unexpected token"),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(&token_type) {
            self.advance();
        } else {
            panic!("Unexpected token: {:?}, {}", self.peek(), message)
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
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        *self.peek().type_info() != TokenType::EndOfFile
    }
}
