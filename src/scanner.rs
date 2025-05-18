use std::any::Any;

use crate::{
    errors::error,
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
            '!' => self.parse_one_or_two_token(TokenType::BANG, TokenType::BANG_EQUAL, '='),
            '=' => self.parse_one_or_two_token(TokenType::EQUAL, TokenType::EQUAL_EQUAL, '='),
            '<' => self.parse_one_or_two_token(TokenType::LESS, TokenType::LESS_EQUAL, '='),
            '>' => self.parse_one_or_two_token(TokenType::GREATER, TokenType::GREATER_EQUAL, '='),
            '/' => self.parse_slash(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {} // do nothing
            _ => error(self.line, String::from("Unexpected character.")),
        }
    }

    fn parse_slash(&mut self) {
        if self.match_next('/') {
            loop {
                if !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                } else {
                    break;
                }
            }
        } else {
            self.add_basic_token(TokenType::SLASH)
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            *self.source.get(self.current).unwrap()
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
}
