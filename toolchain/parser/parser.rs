use crate::parser::nodes::{Expr, Stmt};
use crate::lexer::{Lexed, Token, TokenKind};
use crate::handle_error::{Error, ErrorKind};

pub struct Parsed {
    pub stmts: Option<Vec<Stmt>>,
    pub errors: Vec<Error>,
}

pub fn parsable(lexed: &Lexed) -> bool {
    lexed.errors.is_empty()
}

pub fn parse(lexed: &Lexed) -> Parsed {
    if parsable(lexed) {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        let mut iter = lexed.tokens.iter().peekable();
        let mut prev_token: Option<&Token> = None;

        while let Some(token) = iter.next() {
            match token.kind {
                TokenKind::Let => {
                    if let Some(name_token) = iter.next() {
                        if name_token.kind == TokenKind::Identifier {
                            if let Some(eq_token) = iter.next() {
                                if eq_token.kind == TokenKind::Equal {
                                    if let Some(value_token) = iter.next() {
                                        let value_expr = Expr::Literal {
                                            kind: value_token.kind.clone(),
                                            value: value_token.value.clone(),
                                        };
                                        stmts.push(Stmt::LetStmt {
                                            name: name_token.value.clone(),
                                            value: value_expr,
                                        });
                                    } else {
                                        errors.push(Error::new(
                                            ErrorKind::UnexpectedEndOfInput,
                                            "Expected value after '='".to_string(),
                                            eq_token.line,
                                            eq_token.column,
                                        ));
                                    }
                                } else {
                                    errors.push(Error::new(
                                        ErrorKind::UnexpectedToken,
                                        format!("Expected '=', found {:?}", eq_token.kind),
                                        eq_token.line,
                                        eq_token.column,
                                    ));
                                }
                            } else {
                                errors.push(Error::new(
                                    ErrorKind::UnexpectedEndOfInput,
                                    "Expected '=' after variable name".to_string(),
                                    name_token.line,
                                    name_token.column,
                                ));
                            }
                        } else {
                            errors.push(Error::new(
                                ErrorKind::UnexpectedToken,
                                format!("Expected identifier, found {:?}", name_token.kind),
                                name_token.line,
                                name_token.column,
                            ));
                        }
                    } else {
                        let (line, column) = iter.peek()
                            .map(|t| (t.line, t.column))
                            .unwrap_or((0, 0));
                        errors.push(Error::new(
                            ErrorKind::UnexpectedEndOfInput,
                            "Expected variable name after 'let'".to_string(),
                            line,
                            column,
                        ));
                    }
                }
                TokenKind::Use => {
                    if let Some(stator_token) = iter.next() {
                        if stator_token.kind == TokenKind::Identifier {
                            if let Some(open_square_token) = iter.next() {
                                if open_square_token.kind == TokenKind::LSquare {
                                    let mut substators = Vec::new();

                                    while let Some(token) = iter.next() {
                                        match token.kind {
                                            TokenKind::Identifier => substators.push(token.clone()),
                                            TokenKind::RSquare => break,
                                            TokenKind::Comma => continue,
                                            _ => {
                                                errors.push(Error::new(
                                                    ErrorKind::UnexpectedToken,
                                                    format!("Unexpected token in use imports: {:?}", token.kind),
                                                    token.line,
                                                    token.column,
                                                ));
                                                break;
                                            }
                                        }
                                    }

                                    stmts.push(Stmt::UseStmt {
                                        stator: stator_token.value.clone(),
                                        imports: substators.into_iter().map(|t| Expr::Literal {
                                            kind: t.kind.clone(),
                                            value: t.value.clone(),
                                        }).collect::<Vec<Expr>>(),
                                    });
                                } else {
                                    errors.push(Error::new(
                                        ErrorKind::UnexpectedToken,
                                        format!("Expected '[' after use stator, found {:?}", open_square_token.kind),
                                        open_square_token.line,
                                        open_square_token.column,
                                    ));
                                }
                            } else {
                                errors.push(Error::new(
                                    ErrorKind::UnexpectedEndOfInput,
                                    "Expected '[' after use stator".to_string(),
                                    stator_token.line,
                                    stator_token.column,
                                ));
                            }
                        } else {
                            errors.push(Error::new(
                                ErrorKind::UnexpectedToken,
                                format!("Expected identifier after 'use', found {:?}", stator_token.kind),
                                stator_token.line,
                                stator_token.column,
                            ));
                        }
                    } else {
                        errors.push(Error::new(
                            ErrorKind::UnexpectedEndOfInput,
                            "Expected identifier after 'use'".to_string(),
                            token.line,
                            token.column,
                        ));
                    }
                }
                TokenKind::Dot => {
                    errors.push(Error::new(
                        ErrorKind::UnexpectedToken,
                        "Unexpected dot token".to_string(),
                        token.line,
                        token.column,
                    ));
                }
                _ => {}
            }
            prev_token = Some(token);
        }

        Parsed {
            stmts: Some(stmts),
            errors,
        }
    } else {
        Parsed {
            stmts: None,
            errors: lexed.errors.iter().map(|msg| Error::new(
                ErrorKind::LexError,
                msg.clone(),
                0,
                0,
            )).collect(),
        }
    }
}
