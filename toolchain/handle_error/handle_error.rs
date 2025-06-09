// Copyright (c) 2025, Rotor Language Project
// All rights reserved.

/// Represents the severity level of an error or log message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}
#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidToken,
    UnknownIdentifier,
    UnexpectedToken,
    LexError,
    UnexpectedEndOfInput,
    InvalidEscapeSequence,
    UnterminatedString,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    line: usize,
    column: usize,
    file_name: String
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>, line: usize, column: usize) -> Self {
        Error {
            kind,
            message: message.into(),
            line,
            column,
            file_name: String::new(),
        }
    }

    pub fn push_new(&self, destination: &mut Vec<String>) {
        destination.push(self.message.clone());
    }
}