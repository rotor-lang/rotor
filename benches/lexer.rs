use rotor::lexer::{lex, Lexed};

fn main() {
    divan::main();
}

#[divan::bench(
    args = [
        "let x = 5",
        "(x + y) / [5 - 6] * {fn(x), fn(y)};",
        "",
        "123",
        "foo_bar",
        "let s = \"string with spaces\";",
        "x <= 42 // comment\nlet uwusenpai = \"uwu senpai\""
    ]
)]
fn test_lex(src: &str) -> Lexed {
    lex(src)
}
