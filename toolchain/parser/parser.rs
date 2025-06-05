use crate::parser::nodes::{Expr, Stmt};
use crate::lexer::{Lexed, Token, TokenKind};
use crate::handle_error::{Error, ErrorKind};

pub fn parse(tokens: Lexed) -> Parsed {
    // Assums tokens are not problematic
    if !tokens.errors.is_empty() {
        return Parsed {
            stmts: vec![],
            errors: tokens.errors,
        };
    }
    let mut tree = vec![];
    let mut errors = vec![];
    let mut iter = tokens.tokens.into_iter().peekable();
}