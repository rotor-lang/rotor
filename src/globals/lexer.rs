pub enum TokenKind {
    // Keywords
    Let,
    
    // Identifiers
    Identifier,

    // Literals
    I32,
    
    // Operators
    Equal,

    // Types
    Integer,

    // Punctuation
    Semicolon
}

struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
    column: usize,
}

struct Program {
    tokens: Vec<Token>
}

impl Token {
    fn new(kind: TokenKind, value: String, line: usize, column: usize) -> Self {
        Token { kind, value, line, column }
    }
    fn is_valid(&self) -> bool {
        match self.kind {
            TokenKind::Let => self.value == "let" && self.line > 0 && self.column > 0,
            TokenKind::Identifier => !self.value.is_empty() && self.line > 0 && self.column > 0,
            TokenKind::I32 => self.value == "i32" && self.line > 0 && self.column > 0,
            TokenKind::Equal => self.value == "=" && self.line > 0 && self.column > 0,
            TokenKind::Integer => self.value.parse::<i32>().is_ok() && self.line > 0 && self.column > 0,
            TokenKind::Semicolon => self.value == ";" && self.line > 0 && self.column > 0,
            _ => false
        }
    }
    fn get_debug_info(&self) -> String {
        format!("Token: {:?}, Value: {}, Line: {}, Column: {}", self.kind, self.value, self.line, self.column)
    }
}

// Lexer itself

fn lex(source: &str) -> Program {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;
    
    for (index, ch) in source.chars().enumerate() {
        
    }

}