use std::any::Any;

use crate::{
    errors::{self, error},
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,   // first character in lexeme being scanned
    current: usize, // current character being scanned
    line: u32,      // which line in source code we are scanning
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EndOfFIle,
            String::from(""),
            None,
            self.line,
        ));

        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_basic_token(TokenType::LeftParen),
            ')' => self.add_basic_token(TokenType::RightParen),
            '{' => self.add_basic_token(TokenType::LeftBrace),
            '}' => self.add_basic_token(TokenType::RightBrace),
            ',' => self.add_basic_token(TokenType::Comma),
            '.' => self.add_basic_token(TokenType::Dot),
            '-' => self.add_basic_token(TokenType::Minus),
            '+' => self.add_basic_token(TokenType::Plus),
            ';' => self.add_basic_token(TokenType::Semicolon),
            '*' => self.add_basic_token(TokenType::Star),
            '!' => self.parse_one_or_two_token(TokenType::Bang, TokenType::BangEqual, '='),
            '=' => self.parse_one_or_two_token(TokenType::Equal, TokenType::EqualEqual, '='),
            '<' => self.parse_one_or_two_token(TokenType::Less, TokenType::LessEqual, '='),
            '>' => self.parse_one_or_two_token(TokenType::Greater, TokenType::GreaterEqual, '='),
            '/' => self.parse_slash(),
            '"' => self.parse_string(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {} // do nothing
            _ => {
                if c.is_digit(10) {
                    self.parse_number();
                } else if c.is_alphanumeric() {
                    self.parse_identifier()
                } else {
                    error(self.line, String::from("Unexpected character."))
                }
            }
        }
    }

    fn parse_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let identifier = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        if let Some(which) = self.match_identifier(identifier.as_str()) {
            self.add_token(which, Some(Box::new(identifier.clone())));
        } else {
            self.add_token(TokenType::Identifier, Some(Box::new(identifier)));
        }
    }

    fn parse_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let n = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>();

        if let Ok(n) = n {
            self.add_token(TokenType::Number, Some(Box::new(n)));
        } else {
            errors::error(self.line, String::from("Unable to parse number."));
        }
    }

    fn parse_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            errors::error(self.line, String::from("Unterminated string."))
        }

        // advance past closing quotation mark
        self.advance();

        // trim surrounding quotation marks and turn into String
        let text = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        self.add_token(TokenType::String, Some(Box::new(text)));
    }

    fn parse_slash(&mut self) {
        if self.match_next('/') {
            while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
        } else {
            self.add_basic_token(TokenType::Slash)
        }
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            '\0'
        } else {
            *self.source.get(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            *self.source.get(self.current + 1).unwrap()
        }
    }

    fn parse_one_or_two_token(&mut self, default: TokenType, other: TokenType, next_char: char) {
        let token_type = if self.match_next(next_char) {
            self.advance();
            other
        } else {
            default
        };

        self.add_basic_token(token_type);
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            match self.source.get(self.current) {
                Some(c) => *c == expected,
                None => false,
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.get(self.current).unwrap();
        self.current += 1;
        *c
    }

    fn add_basic_token(&mut self, which: TokenType) {
        self.add_token(which, None);
    }

    fn add_token(&mut self, which: TokenType, literal: Option<Box<dyn Any>>) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(which, text, literal, self.line));
    }

    fn match_identifier(&self, identifier: &str) -> Option<TokenType> {
        match identifier {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
