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
    pub fn expect_either(&mut self, kind: Vec<TokenKind>) -> Result<Token, Error> {
        match self.next() {
            Some(token) if kind.contains(&token.kind) => Ok(token),
            Some(tok) => Err(Error::new(
                ErrorKind::UnexpectedToken,
                format!("Expected either {:?}, found {}", kind, tok.kind),
                tok.line,
                tok.column
            )),
            None => Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("Expected either {:?}, found end of file", kind),
                0,
                0
            ))
        }
    }
}

// Parsing helpers
///////////////////////////////////////////////////////////////////////////////////
pub fn p_LetStmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: (let|const) name[: type] = expr;
    stream.expect_either(vec![TokenKind::Let, TokenKind::Const]);

    let name = stream.expect(TokenKind::Identifier)?;
    let mut ty = None;

    if let Some(colon) = stream.peek() {
        if colon.kind == TokenKind::Colon {
            stream.next();
            let ty_token = stream.expect_either(vec![
                TokenKind::I32,
                TokenKind::BOOL,
                TokenKind::STR
            ])?;
            ty = Some(ty_token.kind);
        }
    }

    stream.expect(TokenKind::Equal)?;

    let valueToken = stream.expect(TokenKind::Integer)?;
    //TODO: Add expression parsing
    // For now, it's only integers
    let value = Box::new(Expr::Literal {
        kind: TokenKind::Integer,
        value: valueToken.value
    });

    // Can't forget the semi-colon
    stream.expect(TokenKind::Semicolon)?;

    Ok(Stmt::LetStmt {
        name: name.value,
        ty,
        value,
    })
}  