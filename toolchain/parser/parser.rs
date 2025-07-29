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

pub fn p_let_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: (let|const) name[: type] = expr;
    stream.expect_either(vec![TokenKind::Let, TokenKind::Const])?;

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

pub fn p_use_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: use stator [(import|*), [import], [import], ...]
    stream.expect(TokenKind::Use)?;
    let stator = stream.expect(TokenKind::Identifier);
    
    // Open square
    stream.expect(TokenKind::LSquare)?;

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
        } else if curr.kind == TokenKind::Star {
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

pub fn p_if_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: if expr { stmt; [stmt;]...} [else {stmt; [stmt;]...}]
    stream.expect(TokenKind::If)?;

    // Again, I don't have expr parsing yet, so it will just be a literal for now
    let cond = Box::new(Expr::Literal {
        kind: TokenKind::Boolean,
        value: "true".to_string()
    });

    stream.expect(TokenKind::LCurly)?;

    #[allow(unused_mut)]
    let mut then_stmts: Vec<Stmt> = vec![];

    // No statement parsing yet

    stream.expect(TokenKind::LCurly)?;
    stream.expect(TokenKind::RCurly)?;

    if let Some(else_token) = stream.peek() {
        if else_token.kind == TokenKind::Else {
            stream.next();

            stream.expect(TokenKind::LCurly)?;
            stream.expect(TokenKind::RCurly)?;
        }
    }

    Ok(Stmt::IfStmt {
        condition: cond,
        then_branch: then_stmts,
        else_branch: None,
    })
}

pub fn p_for_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: for variable in Iterable { stmt; [stmt;]...}
    stream.expect(TokenKind::For)?;

    let var = stream.expect(TokenKind::Identifier)?;
    
    // For now, we only assume the loop is over a collection
    // which has been moved into the variable
    stream.expect(TokenKind::In)?;
    let iterable = stream.expect(TokenKind::Identifier)?;

    // Enter loop body
    stream.expect(TokenKind::LCurly)?;
    
    #[allow(unused_mut)]
    let mut body_stmts: Vec<Stmt> = vec![];

    // Still no block parsing, so we are just having an empty body
    stream.expect(TokenKind::RCurly)?;

    Ok(Stmt::ForStmt {
        variable: var.value,
        iterable: Box::new(Expr::Variable {
            name: iterable.value,
            ty: None // No type inference yet
        }),
        body: body_stmts
    })

}

pub fn p_while_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> {
    // syntax: while condition {stmt; [stmt;]...}
    stream.expect(TokenKind::While)?;

    let condition_token = stream.expect(TokenKind::Boolean)?;
    let condition: Box<Expr> = Box::new(Expr::Literal {
        kind: TokenKind::Boolean,
        value: condition_token.value,
    });

    stream.expect(TokenKind::LCurly)?;

    #[allow(unused_mut)]
    let mut body_stmts: Vec<Stmt> = vec![];
    // ditto

    stream.expect(TokenKind::RCurly)?;
    Ok(Stmt::WhileStmt {
        condition,
        body: body_stmts,
    })
}

// Special parsing
pub fn parse_stmt(stream: &mut TokenStream) -> Result<Stmt, Error> { // TODO: Find the type to be returned
    // This is for parsing all the statements together
    // Contributers, if you add a new statement, please follow
    // the patterns below to create one. Ensure you have made the
    // 'p_{name}_stmt' before adding it.
    match stream.peek().unwrap().kind {
        TokenKind::Let | TokenKind::Const => {
            p_let_stmt(stream)
        },
        TokenKind::Use => {
            p_use_stmt(stream)
        },
        TokenKind::If => {
            p_if_stmt(stream)
        },
        TokenKind::For => {
            p_for_stmt(stream)
        },
        TokenKind::While => {
            p_while_stmt(stream)
        },
        _ => {
            // Currently, most of rotor's code will end up here
            // because it is still in development.
            Err(Error {
                kind: ErrorKind::InvalidToken,
                message: "Found invalid token".to_string(), // TODO: Make a better error message
                line: stream.peek().unwrap().line,
                column: stream.peek().unwrap().column
            })
        }
    }
}