use std::any::Any;

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // first character in lexeme being scanned
    current: usize, // current character being scanned
    line: u32,      // which line in source code we are scanning
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
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
            TokenType::EOF,
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
            '(' => self.add_basic_token(TokenType::LEFT_PAREN),
            ')' => self.add_basic_token(TokenType::RIGHT_PAREN),
            '{' => self.add_basic_token(TokenType::LEFT_BRACE),
            '}' => self.add_basic_token(TokenType::RIGHT_BRACE),
            ',' => self.add_basic_token(TokenType::COMMA),
            '.' => self.add_basic_token(TokenType::DOT),
            '-' => self.add_basic_token(TokenType::MINUS),
            '+' => self.add_basic_token(TokenType::PLUS),
            ';' => self.add_basic_token(TokenType::SEMICOLON),
            '*' => self.add_basic_token(TokenType::STAR),
            _ => {} // not yet
        }
    }

    fn advance(&mut self) -> char {
        println!("{:?}", self.source.chars().nth(self.current));
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_basic_token(&mut self, which: TokenType) {
        self.add_token(which, None);
    }

    fn add_token(&mut self, which: TokenType, literal: Option<Box<dyn Any>>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(which, text, literal, self.line));
    }
}
