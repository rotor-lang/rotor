// Copyright (c) 2025, Rotor Language Project
// All rights reserved.
use std::fmt;
use crate::handle_error::{ErrorKind, Error};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords

    // Variable-related
    Let,
    Const,

    // Dependency-related
    Use,

    // Control flow
    If,
    Else,

    // Repeaters
    For,
    While,

    // Misc
    In,

    // Types
    I32,
    BOOL,
    STR,
    

    // Identifiers & Literals
    Identifier,
    Integer,
    String, // In the future, may require extra data for string type (e.g. raw, format, etc.)
    Float,
    Boolean,

    // Symbols
    Dot,
    Range,
    Equal,
    Semicolon,
    Colon,
    Newline,
    Comma,

    // Parentheses
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,

    // Operators
    Plus,
    Line,
    Star,
    Slash,
    Modulus,
    And,
    Or,
    Not,

    // Comparison
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    EqualEqual,
    NotEqual,

}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pos: usize
}

pub struct Lexed {
    pub tokens: Vec<Token>,
    pub errors: Vec<String>
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Let => "let",
            TokenKind::Const => "const",
            TokenKind::Use => "use",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::In => "in",

            TokenKind::I32 => "i32",
            TokenKind::BOOL => "bool",
            TokenKind::STR => "str",

            TokenKind::Identifier => "identifier",
            TokenKind::Integer => "integer",
            TokenKind::String => "string",
            TokenKind::Float => "float",
            TokenKind::Boolean => "boolean",

            TokenKind::Dot => ".",
            TokenKind::Range => "..",
            TokenKind::Equal => "=",
            TokenKind::Semicolon => ";",
            TokenKind::Colon => ":",
            TokenKind::Newline => "\\n",
            TokenKind::Comma => ",",

            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LCurly => "{",
            TokenKind::RCurly => "}",
            TokenKind::LSquare => "[",
            TokenKind::RSquare => "]",

            TokenKind::Plus => "+",
            TokenKind::Line => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Modulus => "%",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Not => "!",

            TokenKind::GreaterThan => ">",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::EqualEqual => "==",
            TokenKind::NotEqual => "!=",
        };
        write!(f, "{}", s)
    }
}

impl Lexed {
    pub fn get_debug_info(&self) {
        for token in &self.tokens {
            println!("{}", token.get_debug_info());
        }
    }
    pub fn is_working(&self) -> bool {
        self.errors.is_empty()
        ||
        self.tokens.iter().all(|token| token.is_valid())
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: impl Into<String>, line: usize, column: usize, pos: usize ) -> Self {
        Token {
            kind,
            value: value.into(),
            line,
            column,
            pos
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.kind {
            TokenKind::Let => self.value == "let",
            TokenKind::Const => self.value == "const",
            TokenKind::Use => self.value == "use",
            TokenKind::If => self.value == "if",
            TokenKind::Else => self.value == "else",
            TokenKind::For => self.value == "for",
            TokenKind::While => self.value == "while",
            TokenKind::In => self.value == "in",
            TokenKind::Identifier => !self.value.is_empty(),
            TokenKind::I32 => self.value == "i32",
            TokenKind::STR => self.value == "str",
            TokenKind::Equal => self.value == "=",
            TokenKind::Integer => self.value.parse::<i32>().is_ok(),
            TokenKind::Float => self.value.parse::<f32>().is_ok(),
            TokenKind::String => self.value.starts_with('"') && self.value.ends_with('"'),
            TokenKind::BOOL => self.value == "bool",
            TokenKind::Boolean => self.value == "true" || self.value == "false",
            TokenKind::Dot => self.value == ".",
            TokenKind::Range => self.value == "..",
            TokenKind::Semicolon => self.value == ";",
            TokenKind::Colon => self.value == ":",
            TokenKind::Comma => self.value == ",",
            TokenKind::Newline => self.value == "\n",
            TokenKind::LParen => self.value == "(",
            TokenKind::RParen => self.value == ")",
            TokenKind::LCurly => self.value == "{",
            TokenKind::RCurly => self.value == "}",
            TokenKind::LSquare => self.value == "[",
            TokenKind::RSquare => self.value == "]",
            TokenKind::Plus => self.value == "+",
            TokenKind::Line => self.value == "-",
            TokenKind::Star => self.value == "*",
            TokenKind::Slash => self.value == "/",
            TokenKind::Modulus => self.value == "%",
            TokenKind::And => self.value == "&&",
            TokenKind::Or => self.value == "||",
            TokenKind::Not => self.value == "!",
            TokenKind::GreaterThan => self.value == ">",
            TokenKind::LessThan => self.value == "<",
            TokenKind::GreaterThanOrEqual => self.value == ">=",
            TokenKind::LessThanOrEqual => self.value == "<=",
            TokenKind::EqualEqual => self.value == "==",
            TokenKind::NotEqual => self.value == "!=",
        }
    }

    pub fn get_debug_info(&self) -> String {
        format!(
            "Token: {:?}, Value: '{}', Line: {}, Column: {}",
            self.kind, self.value, self.line, self.column
        )
    }
}

pub fn lex(source: &str) -> Lexed {
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut pos: usize = 0;
    let chars: &[u8] = source.as_bytes();

    while pos < chars.len() {
        let ch: char = chars[pos] as char;

        match ch {
            // WARNING: 
            // The following match arms are not special
            // so they only need to be eaten and not
            // have any extra logic for them.
            '=' | ';' | ':' | '(' | ')' | '{' | '}' | '[' | ']' | '+' | '-' | '*' | '%' => {
                let kind = match ch {
                    '=' => TokenKind::Equal,
                    ';' => TokenKind::Semicolon,
                    ':' => TokenKind::Colon,
                    '(' => TokenKind::LParen,
                    ')' => TokenKind::RParen,
                    '{' => TokenKind::LCurly,
                    '}' => TokenKind::RCurly,
                    '[' => TokenKind::LSquare,
                    ']' => TokenKind::RSquare,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Line,
                    '*' => TokenKind::Star,
                    '%' => TokenKind::Modulus,
                    _ => unreachable!(), // Oopsie daisys, you shouldn't be here. Now suffer a terrible error message.
                    
                };
                tokens.push(Token::new(kind, ch.to_string(), line, column, pos));
                pos += 1;
                column += 1;
            }
            '/' => {
                if pos + 1 < chars.len() && chars[pos + 1] == b'/' {
                    // Single-line comment
                    pos += 2;
                    column += 2;
                    while pos < chars.len() && chars[pos] != b'\n' {
                        pos += 1;
                        column += 1;
                    }
                } else if pos + 1 < chars.len() && chars[pos + 1] == b'*' {
                    // Multi-line comment
                    pos += 2;
                    column += 2;
                    while pos < chars.len() {
                        if pos + 1 < chars.len() && chars[pos] == b'*' && chars[pos + 1] == b'/' {
                            pos += 2;
                            column += 2;
                            break;
                        }
                        if chars[pos] == b'\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                        pos += 1;
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Slash, "/", line, column, pos));
                    pos += 1;
                    column += 1;
                }
            }
            // The '.' character is already handled in the first match arm above.
            c if c.is_alphabetic() || c == '_' => {
                let start_column = column;
                let start_pos: usize = pos;
                let mut identifier = String::new();

                while pos < chars.len() && ((chars[pos] as char).is_alphanumeric() || chars[pos] == b'_') {
                    identifier.push(chars[pos] as char);
                    pos += 1;
                    column += 1;
                }

                match identifier.as_str() {
                    "let" => tokens.push(Token::new(TokenKind::Let, identifier, line, start_column, start_pos)),
                    "const" => tokens.push(Token::new(TokenKind::Const, identifier, line, start_column, start_pos)),
                    "if" => tokens.push(Token::new(TokenKind::If, identifier, line, start_column, start_pos)),
                    "else" => tokens.push(Token::new(TokenKind::Else, identifier, line, start_column, start_pos)),
                    "for" => tokens.push(Token::new(TokenKind::For, identifier, line, start_column, start_pos)),
                    "while" => tokens.push(Token::new(TokenKind::While, identifier, line, start_column, start_pos)),
                    "in" => tokens.push(Token::new(TokenKind::In, identifier, line, start_column, start_pos)),
                    "i32" => tokens.push(Token::new(TokenKind::I32, identifier, line, start_column, start_pos)),
                    "f32" => tokens.push(Token::new(TokenKind::Float, identifier, line, start_column, start_pos)),
                    "bool" => tokens.push(Token::new(TokenKind::BOOL, identifier, line, start_column, start_pos)),
                    "str" => tokens.push(Token::new(TokenKind::STR, identifier, line, start_column, start_pos)),
                    "true" => tokens.push(Token::new(TokenKind::Boolean, identifier, line, start_column, start_pos)),
                    "false" => tokens.push(Token::new(TokenKind::Boolean, identifier, line, start_column, start_pos)),
                    "use" => tokens.push(Token::new(TokenKind::Use, identifier, line, start_column, start_pos)),
                    _ => tokens.push(Token::new(TokenKind::Identifier, identifier, line, start_column, start_pos)),
                }
            }
            c if c.is_digit(10) => {
                let start_column = column;
                let start_pos: usize = pos; // i feel like eating a jobonga. you don't know what that is? uncultured -_-
                let mut number = String::new();
                let mut other_numeric_type: TokenKind = TokenKind::Integer;

                while pos < chars.len() && (chars[pos] as char).is_digit(10) {
                    if chars[pos] == b'.' && other_numeric_type == TokenKind::Integer {
                        other_numeric_type = TokenKind::Float;
                        number.push('.');
                        pos += 1;
                        column += 1;
                    } else {
                        number.push(chars[pos] as char);
                        pos += 1;
                        column += 1;
                    }
                    
                }

                tokens.push(Token::new(other_numeric_type, number, line, start_column, start_pos));
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    tokens.push(Token::new(TokenKind::Newline, "\n", line, column, pos));
                    line += 1;
                    column = 1;
                } else if c == '\t' {
                    column += 4; // This is assuming a tab will be 4 spaces. idk anyone who actually has a tab set to 8 spaces. if you do, you should probably call emergency services and get help.
                } else {
                    column += 1;
                }
                pos += 1;
            }

            '\"' => {
                let start_column = column;
                let mut string = String::new();
                pos += 1;
                column += 1;
                while pos < chars.len() && chars[pos] != b'\"' {
                    string.push(chars[pos] as char);
                    pos += 1;
                    column += 1;
                }
                // Eat the closing quote
                column += 1;
                pos += 1;
                tokens.push(Token::new(TokenKind::String, string, line, start_column, pos));
            }

            _ => {
                Error::new(
                    ErrorKind::InvalidToken,
                    format!("Invalid token({}, {}): {}", line, column, ch), //                      o        o
                    line, //                                                                                     |               <- this is gart. dont be mean to him.
                    column, //                                                                                 \___/                he's really nice and helpful.
                ).push_new(&mut errors);  //                                                                           so don't hurt him or i will hurt you. >:(
                pos += 1;
                column += 1;
            }
        }
    }
    Lexed { tokens, errors }
}
