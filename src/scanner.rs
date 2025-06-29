use std::collections::HashMap;

use crate::{
    errors,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,  // source code as a vector of characters
    tokens: Vec<Token>, // vector of tokens produced
    start: usize,       // first character in lexeme being scanned
    current: usize,     // current character being scanned
    line: u32,          // which line in source code we are scanning
    reserved_keywords: HashMap<&'static str, TokenType>, // keywords reserved by Lox
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut reserved: HashMap<&'static str, TokenType> = HashMap::new();
        reserved.insert("and", TokenType::And);
        reserved.insert("class", TokenType::Class);
        reserved.insert("else", TokenType::Else);
        reserved.insert("false", TokenType::False);
        reserved.insert("for", TokenType::For);
        reserved.insert("fun", TokenType::Fun);
        reserved.insert("if", TokenType::If);
        reserved.insert("nil", TokenType::Nil);
        reserved.insert("or", TokenType::Or);
        reserved.insert("print", TokenType::Print);
        reserved.insert("return", TokenType::Return);
        reserved.insert("super", TokenType::Super);
        reserved.insert("this", TokenType::This);
        reserved.insert("true", TokenType::True);
        reserved.insert("var", TokenType::Var);
        reserved.insert("while", TokenType::While);

        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            reserved_keywords: reserved,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EndOfFile,
            String::from(""),
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
            '!' => self.parse_with_next_char(TokenType::Bang, TokenType::BangEqual, '='),
            '=' => self.parse_with_next_char(TokenType::Equal, TokenType::EqualEqual, '='),
            '<' => self.parse_with_next_char(TokenType::Less, TokenType::LessEqual, '='),
            '>' => self.parse_with_next_char(TokenType::Greater, TokenType::GreaterEqual, '='),
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
                    errors::error(self.line, String::from("Unexpected character."))
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

        if let Some(which) = self.reserved_keywords.get(identifier.as_str()) {
            self.add_token(which.clone());
        } else {
            self.add_token(TokenType::Identifier(identifier));
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
            self.add_token(TokenType::Number(n));
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

        self.add_token(TokenType::Str(text));
    }

    fn parse_block_comment(&mut self) {
        // shed '*'
        self.advance();

        while !self.is_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                self.parse_block_comment();
            } else {
                self.advance();
            }
        }

        if !self.is_at_end() {
            // shed '*'
            self.advance();
            // shed '/'
            self.advance();
        }
    }

    fn parse_slash(&mut self) {
        if self.match_next('/') {
            while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
        } else if self.match_next('*') {
            self.parse_block_comment();
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

    fn parse_with_next_char(&mut self, default: TokenType, other: TokenType, next_char: char) {
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
        self.add_token(which);
    }

    fn add_token(&mut self, which: TokenType) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens.push(Token::new(which, text, self.line));
    }
}
