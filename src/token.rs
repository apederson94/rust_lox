#[derive(Debug, Clone)]
pub struct Token {
    which: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(which: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            which,
            lexeme,
            line,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn which(&self) -> &TokenType {
        &self.which
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {}", self.which, self.lexeme)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl TokenType {}
