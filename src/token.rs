use std::any::Any;

#[derive(Debug)]
pub struct Token {
    which: TokenType,
    lexeme: String,
    literal: Option<Box<dyn Any>>,
    line: u32,
}

impl Token {
    pub fn new(which: TokenType, lexeme: String, literal: Option<Box<dyn Any>>, line: u32) -> Self {
        Self {
            which,
            lexeme,
            literal: literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.which, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, Copy)]
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

    // Literals
    Identifier,
    String,
    Number,

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
    EndOfFIle,
}
