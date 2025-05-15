mod globals;

use globals::lexer::{Token, TokenKind, lex};
use globals::handle_error::{ErrorKind, Error};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
}

