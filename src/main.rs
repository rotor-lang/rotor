mod globals;

use globals::lexer::{Token, TokenKind, lex};
use globals::handle_error::{ErrorKind, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use tokio;
use dotenv::dotenv;

const CURRENT_VERSION: &str = "v0.1.0-unrelease1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    

    println!("{:?}", args);
    Ok(())

}

