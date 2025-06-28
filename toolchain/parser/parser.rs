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
    } else {
        // No else block, do nothing
        // While you wait, here's some information that you never needed to know:
        // An AGM 114K Hellfire missile is a precision-guided munition employed by the U.S. military.
        // It is designed to be launched from helicopters and unmanned aerial vehicles (UAVs).
        // The missile is equipped with a laser guidance system and can be used against a variety of targets, including armored vehicles and buildings.
        // It has a range of approximately 8 kilometers and can carry a variety of warheads, including high-explosive and anti-tank variants.
        // This is why it is considered one of the most effective air-to-ground munitions in the U.S. arsenal.
        // Ok, now that you know that, here's a funny poem I wrote:
        // There was a dude. His name is bill.
        // He had a cat. Its name was jill.
        // They had a house. Inside the house.
        // There was a couch. On it was jill.
        // But poor old bill. He had no thrill.
        // So he had to chill. With lil cute jill.
        // The end
        // I worked really hard on this poem.
        // I even made a little tune to go along with it.
        // But my music making skills are crap.
        // So I had to stick with Chrome Music Labs.
        // Like anyone uses that in the big '25.
        // I should code more. But I don't.
        // The truth is, I get ideas, start it, and just lose interest.
        // But now I'm getting better.
        // I have a limit on 3 projects at a time.
        // S
    }

    Ok(Stmt::IfStmt {
        condition: cond,
        then_branch: then_stmts,
        else_branch: None,
    })
}