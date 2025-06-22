use rotor::parser::{TokenStream, p_LetStmt};
use rotor::lexer::lex; // assuming this exists
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench(args=[
    "let x = 15;",
    "const test = 42;",
    "let aBcDeF: i32 = 0;",
    "let a = 1; let b = 2; let c = 3;"
])]
fn test_parse(src: &str) {
    let lexed = lex(src);
    let mut stream = TokenStream::new(lexed.tokens);

    let _ = black_box(p_LetStmt(&mut stream));
}
