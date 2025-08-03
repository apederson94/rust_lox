use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(which: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            token_type: which,
            lexeme,
            line,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn type_info(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line(&self) -> u32 {
        self.line
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    QuestionMark,
    Colon,

    // Literals
    Identifier(String),
    Str(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.token_type {
            TokenType::EndOfFile => "EndOfFile",
            _ => self.lexeme(),
        };
        write!(f, "{}", msg)
    }
}
