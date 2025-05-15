#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidToken,
    UnknownIdentifier,
    UnexpectedToken,
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

    pub fn push_new(&self, destination: &mut Vec<ErrorKind>) {
        destination.push(self.kind.clone());
    }
}