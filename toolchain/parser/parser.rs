use std::iter::Peekable;
use crate::lexer::{Token, TokenKind};

struct TokenStream<I> { iter: Peekable<I> }
impl TokenStream { fn peek(&self) -> Option<&Token>; fn next(&mut self) -> Option<Token>; }
