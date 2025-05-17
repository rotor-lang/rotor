mod globals;

use globals::lexer::{Token, TokenKind, lex};
use globals::handle_error::{ErrorKind, Error};
// use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
// use tokio;
// use dotenv::dotenv;

// const CURRENT_VERSION: &str = "v0.1.0-unrelease1";

//#[tokio::main]
fn main() {
    let my_code = "let x: i32 = 5;";

    let lexed = lex(my_code);

    println!("Lexed Token: {:?}", lexed.tokens);
    println!("Errors: {:?}", lexed.errors);
}

