// Copyright (c) 2025, Rotor Language Project
// All rights reserved.

use crate::globals::handle_error::{ErrorKind, Error};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Let,
    Identifier,
    I32,
    Equal,
    Integer,
    Float,
    String,
    Semicolon,
    Newline
}


#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
    column: usize,
}

pub struct Lexed {
    tokens: Vec<Token>,
    errors: Vec<ErrorKind>
}

impl Lexed {
    pub fn get_debug_info(&self) {
        for token in &self.tokens {
            println!("{}", token.get_debug_info());
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: impl Into<String>, line: usize, column: usize) -> Self {
        Token {
            kind,
            value: value.into(),
            line,
            column,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.kind {
            TokenKind::Let => self.value == "let",
            TokenKind::Identifier => !self.value.is_empty(),
            TokenKind::I32 => self.value == "i32",
            TokenKind::Equal => self.value == "=",
            TokenKind::Integer => self.value.parse::<i32>().is_ok(),
            TokenKind::Float => self.value.parse::<f32>().is_ok(),
            TokenKind::String => self.value.starts_with('"') && self.value.ends_with('"'),
            TokenKind::Semicolon => self.value == ";",
            TokenKind::Newline => self.value == "\n",
        }
    }

    pub fn get_debug_info(&self) -> String {
        format!(
            "Token: {:?}, Value: {}, Line: {}, Column: {}",
            self.kind, self.value, self.line, self.column
        )
    }
}

pub fn lex(source: &str) -> Lexed {
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<ErrorKind> = Vec::new();
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut pos: usize = 0;
    let chars: Vec<char> = source.chars().collect();

    while pos < chars.len() {
        let ch: char = chars[pos];

        match ch {
            '=' => {
                tokens.push(Token::new(TokenKind::Equal, "=", line, column));
                pos += 1;
                column += 1;
            }
            ';' => {
                tokens.push(Token::new(TokenKind::Semicolon, ";", line, column));
                pos += 1;
                column += 1;
            }
            c if c.is_alphabetic() || c == '_' => {
                let start_column = column;
                let mut identifier = String::new();

                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    identifier.push(chars[pos]);
                    pos += 1;
                    column += 1;
                }

                match identifier.as_str() {
                    "let" => tokens.push(Token::new(TokenKind::Let, identifier, line, start_column)),
                    "i32" => tokens.push(Token::new(TokenKind::I32, identifier, line, start_column)),
                    _ => tokens.push(Token::new(TokenKind::Identifier, identifier, line, start_column)),
                }
            }
            c if c.is_digit(10) => {
                let start_column = column;
                let mut number = String::new();
                let mut other_numeric_type: TokenKind = TokenKind::Integer;

                while pos < chars.len() && chars[pos].is_digit(10) {
                    if chars[pos] == '.' && other_numeric_type == TokenKind::Integer {
                        other_numeric_type = TokenKind::Float;
                        number.push('.');
                        pos += 1;
                        column += 1;
                    } else {
                        number.push(chars[pos]);
                        pos += 1;
                        column += 1;
                    }
                    
                }

                tokens.push(Token::new(other_numeric_type, number, line, start_column));
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    tokens.push(Token::new(TokenKind::Newline, "\n", line, column));
                    line += 1;
                    column = 1;
                } else if c == '\t' {
                    column += 4; // This is assuming a tab will be 4 spaces. idk anyone who actually has a tab set to 8 spaces. if you do, you should probably call emergency services and get help.
                } else {
                    column += 1;
                }
                pos += 1;
            }
            _ => {
                Error::new(
                    ErrorKind::InvalidToken,
                    format!("Invalid token: {}", ch),
                    line,
                    column,
                ).push_new(&mut errors);
                pos += 1;
                column += 1;
            }
        }
    }

    Lexed { tokens, errors }
}

