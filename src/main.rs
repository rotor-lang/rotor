mod globals;

use globals::lexer::{lex};
use globals::handle_error::{ErrorKind, Error};
// use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
// use tokio;
// use dotenv::dotenv;

// const CURRENT_VERSION: &str = "v0.1.0-unrelease1";

//#[tokio::main]
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    
    if args.len() < 2  {
        println!("Use --help for information on commands.");
    }

    if args[1] == "--help" {
        println!("Usage: rotor [OPTIONS] [FILE]");
        println!("------------------------------------------");
        println!("WARNING: This is an early version of the program, and it is not yet fully functional.");
        println!("Only the --lex option has been implemented so far.");
        println!("------------------------------------------");
        println!("Options:");
        println!("  --help       Show this help message");
        println!("  --version    Show the version of the program");
        println!("  --run        Run the specified file");
        println!("  --compile    Compile the specified file");
        println!("  --debug      Debug the specified file");
        println!("  --lex        Lex the specified file");
        println!("---------------------------------------");
    } else if args[1] == "--version" {
        println!("Version: {}", "v0.1.0-unrelease1.1");
    } else if args[1] == "--run" {
        if args.len() < 3 {
            println!("Running is not supported yet.");
            return;
        }
        let source = std::fs::read_to_string(&args[2]).expect("Unable to read file");
        let lexed = lex(&source);
        lexed.get_debug_info();
    } else if args[1] == "--compile" {
        println!("Compilation is not yet implemented.")
    } else if args[1] == "--debug" {
        println!("Debugging is not yet implemented.")
    } else if args[1] == "--lex" {
        let source = std::fs::read_to_string(&args[2]).expect("Unable to read file");
        let lexed = lex(&source);
        lexed.get_debug_info();
    } else if args[1] == "--test" {
        println!("Testing is not yet implemented.")
    } else if args[1] == "--build" {
        println!("Building is not yet implemented.")
    } else if args[1] == "--install" {
        println!("Installing is not yet implemented.")
    } else {
        println!("Unknown command: {}", args[1]);
    }
}

