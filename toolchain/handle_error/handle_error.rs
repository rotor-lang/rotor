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
    InvalidEscapeSequence,
    UnterminatedString,
}

#[allow(dead_code)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    line: usize,
    column: usize,
    pos: usize,
    file_name: String
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>, line: usize, column: usize, pos: usize) -> Self {
        Error {
            kind,
            message: message.into(),
            line,
            column,
            pos,
            file_name: String::new(),
        }
    }

    pub fn push_new(&self, destination: &mut Vec<String>) {
        destination.push(self.message.clone());
    }
}