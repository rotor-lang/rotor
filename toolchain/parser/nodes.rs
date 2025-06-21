use crate::lexer::TokenKind;

pub enum Expr {
    Literal {
        kind: TokenKind,
        value: String
    },
    Variable {
        name: String,
        // More memory stuff to come like references, pointers, etc
        // But right now, I'm just gonna add the type
        ty: Option<TokenKind>
    },
    BinaryOp {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr> 
    },
    UnaryOp {
        op: TokenKind,
        expr: Box<Expr>
    },
    FnCall {
        name: String,
        args: Vec<Expr>
    },
    Array {
        elements: Vec<Expr>,
        length: Option<usize>
    }
}

pub enum Stmt {
    LetStmt {
        name: String,
        ty: Option<TokenKind>,
        value: Box<Expr>,
    },
    UseStmt {
        stator: String,
        imports: UseImports
    },
    FnDecl {
        name: String,
        params: Vec<FnParam>,
        return_ty: Option<TokenKind>,
        body: Option<Vec<Stmt>>
    },
    CallStmt(Vec<String>)
}

pub enum UseImports {
    List(Vec<String>),
    Wildcard
}

pub struct FnParam {
    name: String,
    ty: TokenKind
}