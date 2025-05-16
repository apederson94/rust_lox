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

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}
