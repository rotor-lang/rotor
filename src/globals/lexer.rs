// Copyright (c) 2025, Rotor Language Project
// All rights reserved.

use crate::globals::handle_error::{ErrorKind, Error};

#[derive(Debug)]
pub enum TokenKind {
    Let,
    Identifier,
    I32,
    Equal,
    Integer,
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

pub struct Program {
    tokens: Vec<Token>,
    errors: Vec<ErrorKind>
}

impl Program {
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

pub fn lex(source: &str) -> Program {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut line = 1;
    let mut column = 1;
    let mut pos = 0;
    let chars: Vec<char> = source.chars().collect();

    while pos < chars.len() {
        let ch = chars[pos];

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

                while pos < chars.len() && chars[pos].is_digit(10) {
                    number.push(chars[pos]);
                    pos += 1;
                    column += 1;
                }

                tokens.push(Token::new(TokenKind::Integer, number, line, start_column));
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    tokens.push(Token::new(TokenKind::Newline, "\n", line, column));
                    line += 1;
                    column = 1;
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

    Program { tokens, errors }
}

