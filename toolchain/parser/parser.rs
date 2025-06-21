use std::iter::Peekable;
use crate::lexer::{Token, TokenKind};
use crate::parser::nodes::{Expr, Stmt};
use crate::handle_error::{ErrorKind, Error};

pub struct TokenStream {
    tokens: Peekable<std::vec::IntoIter<Token>>
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        match self.next() {
            Some(token) if token.kind == kind => Ok(token),
            Some(tok) => Err(Error::new(
                ErrorKind::UnexpectedToken,
                format!("Expected {}, found {}", kind, tok.kind),
                tok.line,
                tok.column
            )),
            None => Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("Expected {}, found end of file", kind),
                0,
                0
            ))
        }
    }
}

// Parsing helpers
///////////////////////////////////////////////////////////////////////////////////
//pub fn p_LetStmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // Coming soon...
//}