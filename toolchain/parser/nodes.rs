use crate::lexer::{TokenKind, Token};
use crate::nodes::*;

pub enum Stmt {
    LetStmt {
        name: String,
        value: Expr,
    },
}

pub enum Expr {
    Literal {
        kind: TokenKind,    
        value: String,      
    },
    Variable {
        name: String,
    },
    BinaryOp {
        left: Box<Expr>,
        op: TokenKind,       
        right: Box<Expr>,
    },
    UnaryOp {
        op: TokenKind,       
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}
