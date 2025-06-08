use crate::lexer::{TokenKind, Token};
use crate::nodes::*;

pub struct Parsed {
    pub tree: Vec<Stmt>,
    pub errors: Vec<Error>,
}

impl Parsed {
    pub fn new(tree: Vec<Stmt>, errors: Vec<Error>) -> Self {
        Parsed { tree, errors }
    }
}

pub struct Path {
    pub parent: String,
    pub children: Option<Vec<String>>,
}



pub enum Stmt {
    LetStmt {
        name: String,
        value: Expr,
    },
    UseStmt {
        stator: String, // Stators are modules
        imports: Array<Literal>,
    },
    CallStmt {
        name: String,
        args: Vec<Expr>,
    },
    FnDecl {
        name: String,
        params: Vec<String>,
        return_t: Option<String>,
        body: Vec<Stmt>,
    }
}

pub enum Expr {
    Literal {
        kind: TokenKind,    
        value: String,      
    },
    Array {
        elements: Vec<Expr>,
        length: Option<usize>,
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
