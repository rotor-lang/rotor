#[derive(Debug)]
pub enum TokenKind {
    Let,
    Identifier,
    I32,
    Equal,
    Integer,
    Semicolon,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
    column: usize,
}

struct Program {
    tokens: Vec<Token>,
}

impl Program {
    fn get_debug_info(&self) {
        for token in &self.tokens {
            println!("{}", token.get_debug_info());
        }
    }
}

impl Token {
    fn new(kind: TokenKind, value: impl Into<String>, line: usize, column: usize) -> Self {
        Token {
            kind,
            value: value.into(),
            line,
            column,
        }
    }

    fn is_valid(&self) -> bool {
        match self.kind {
            TokenKind::Let => self.value == "let",
            TokenKind::Identifier => !self.value.is_empty(),
            TokenKind::I32 => self.value == "i32",
            TokenKind::Equal => self.value == "=",
            TokenKind::Integer => self.value.parse::<i32>().is_ok(),
            TokenKind::Semicolon => self.value == ";",
        }
    }

    fn get_debug_info(&self) -> String {
        format!(
            "Token: {:?}, Value: {}, Line: {}, Column: {}",
            self.kind, self.value, self.line, self.column
        )
    }
}

pub fn lex(source: &str) -> Program {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;

    for ch in source.chars() {
        if ch == '=' {
            tokens.push(Token::new(TokenKind::Equal, "=", line, column));
        }
        column += 1;
    }

    Program { tokens }
}
