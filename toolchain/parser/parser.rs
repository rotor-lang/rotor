use std::iter::Peekable;
use crate::lexer::{Token, TokenKind};
use crate::parser::nodes::{Expr, Stmt, UseImports};
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
// Template for helper functions (if you need it)

/*

pub fn p_STMTNAME(stream: &mut TokenStream) -> Result<Stmt, Error> {
    
}

*/

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

    let value_token = stream.expect(TokenKind::Integer)?;
    //TODO: Add expression parsing
    // For now, it's only integers
    let value = Box::new(Expr::Literal {
        kind: TokenKind::Integer,
        value: value_token.value
    });

    // Can't forget the semi-colon
    stream.expect(TokenKind::Semicolon)?;

    Ok(Stmt::LetStmt {
        name: name.value,
        ty,
        value,
    })
}  

pub fn p_UseStmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: use stator [(import|*), [import], [import], ...]
    stream.expect(TokenKind::Use);
    let stator = stream.expect(TokenKind::Identifier);
    
    // Open square
    stream.expect(TokenKind::LSquare);

    // Loop until found closing square
    // But maybe also wildcard, idk
    let mut imports = UseImports::List(vec![]);
    let mut import_list: Vec<String> = vec![];
    loop {
        let curr = stream.next().unwrap();

        if curr.kind == TokenKind::RSquare {
            // if the wildcard is used, then we dump the list
            if let UseImports::Wildcard = imports {
                return Ok(Stmt::UseStmt { stator: stator.unwrap().value, imports: imports })
            } else {
                return Ok(Stmt::UseStmt { stator: stator.unwrap().value, imports: UseImports::List(import_list) })
            }
        } else if curr.kind == TokenKind::Multiply {
            imports = UseImports::Wildcard;
        } else if curr.kind == TokenKind::Identifier {
            import_list.push(curr.value);
        } else if curr.kind == TokenKind::Comma {
            // do nothing, becuase what is there to do
            continue;
        } else {
            return Err(Error::new(
                ErrorKind::InvalidToken, 
                format!("Expected either ']', ',' or identifier, found {}", curr.kind),
                curr.line,
                curr.column
            ))
        }
    }
}