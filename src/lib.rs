pub mod globals;

pub use globals::lexer::{TokenKind, Token, lex, Lexed};
pub use globals::handle_error::{ErrorKind, Error};
