use crate::parser::nodes::{Expr, Stmt};
use crate::lexer::{Lexed, Token, TokenKind};
use crate::handle_error::{Error, ErrorKind};

pub struct Parsed {
    pub stmts: Option<Vec<Stmt>>,
    pub errors: Vec<Error>,
}

pub fn parsable(lexed: &Lexed) -> bool {
    // Check if the lexed tokens are valid for parsing
    lexed.errors.is_empty();
}

pub fn parse(lexed: &Lexed) -> Parsed {
    if parsable(lexed) {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();

        // Iterate tokens
        let mut iter = lexed.tokens.iter().peekable();

        // Parse tokens into statements
        while let Some(token) = iter.next() {
            match token.kind {
                TokenKind::Let => {
                    // syntax: let <identifier> = <expression>
                    if let Some(name_token) = iter.next() {
                        if let TokenKind::Identifier(name) = name_token.kind {
                            if let Some(eq_token) = iter.next() {
                                if eq_token.kind == TokenKind::Equal {
                                    if let Some(value_token) = iter.next() {
                                        let value_expr = Expr::Literal {
                                            kind: value_token.kind,
                                            value: value_token.value.clone(),
                                        };
                                        stmts.push(Stmt::LetStmt {
                                            name,
                                            value_expr,
                                        });
                                    } else {
                                        errors.push(Error::new(
                                            ErrorKind::UnexpectedEndOfInput,
                                            "Expected value after '='".to_string(),
                                        ));
                                    }
                                } else {
                                    errors.push(Error::new(
                                        ErrorKind::UnexpectedToken,
                                        format!("Expected '=', found {:?}", eq_token.kind),
                                    ));
                                }
                            } else {
                                errors.push(Error::new(
                                    ErrorKind::UnexpectedEndOfInput,
                                    "Expected '=' after variable name".to_string(),
                                ));
                            }
                        } else {
                            errors.push(Error::new(
                                ErrorKind::UnexpectedToken,
                                format!("Expected identifier, found {:?}", name_token.kind),
                            ));
                        }
                    } else {
                        errors.push(Error::new(
                            ErrorKind::UnexpectedEndOfInput,
                            format!(
                                "Expected variable name after 'let', got {:?}",
                                iter.peek().map(|t| &t.kind)
                            ),
                        ));
                    }
                }
                TokenKind::Use => {
                    if let Some(stator_token) = iter.next() {
                        if let TokenKind::Identifier(stator) = stator_token.kind {
                            if let Some(open_square_token) = iter.next() {
                                if let TokenKind::LSquare(_) = open_square_token.kind {
                                    let mut substators = Vec::new();

                                    while let Some(token) = iter.next() {
                                        match token.kind {
                                            TokenKind::Identifier(_) => substators.push(token.clone()),
                                            TokenKind::RSquare => break,
                                            TokenKind::Comma => continue,
                                            _ => {
                                                errors.push(Error::new(
                                                    ErrorKind::UnexpectedToken,
                                                    format!("Unexpected token in use imports: {:?}", token.kind),
                                                ));
                                                break;
                                            }
                                        }
                                    }

                                    stmts.push(Stmt::UseStmt {
                                        stator: stator.clone(),
                                        imports: substators,
                                    });
                                } else {
                                    errors.push(Error::new(
                                        ErrorKind::UnexpectedToken,
                                        format!("Expected '[' after use stator, found {:?}", open_square_token.kind),
                                    ));
                                }
                            } else {
                                errors.push(Error::new(
                                    ErrorKind::UnexpectedEndOfInput,
                                    "Expected '[' after use stator".to_string(),
                                ));
                            }
                        } else {
                            errors.push(Error::new(
                                ErrorKind::UnexpectedToken,
                                format!("Expected identifier after 'use', found {:?}", stator_token.kind),
                            ));
                        }
                    } else {
                        errors.push(Error::new(
                            ErrorKind::UnexpectedEndOfInput,
                            "Expected identifier after 'use'".to_string(),
                        ));
                    }
                }
                TokenKind::Dot => {
                    // syntax: <parent>.<child>...
                    // WARNING: There are two cases for this:
                    // 1. A parent token followed by a dot and then an identifier (e.g., `parent.child`)
                    // 2. A floating point number (e.g., `3.14`)
                    // We need to see if it's an identifier or a number before the dot.
                    let mut object: Path;
                    if let Some(parent_token) = iter.previous() {
                        if let TokenKind::Identifier(parent) = parent_token.kind {
                            object.parent = parent;
                            while iter.peek().kind == TokenKind::Identifier || iter.peek().kind == TokenKind::Dot {
                                if let Some(next_token) = iter.next() {
                                    if let TokenKind::Identifier(child) = next_token.kind {
                                        object.children.push(child)
                                    } else if let TokenKind::Dot(dot) = next_token.kind {
                                        // Do nothing because it's a dot
                                    } else {
                                        errors.push()
                                    }
                                }
                            }
                        } else if let TokenKind::Integer(whole) = parent_token.kind {
                            // Handle float
                        }
                    }
                }
                _ => {}
            }
            let prev_token = token;
        }
        Parsed {
            stmts: Some(stmts),
            errors,
        }
    } else {
        Parsed {
            stmts: None,
            errors: lexed.errors.clone(),
        }
    }
}