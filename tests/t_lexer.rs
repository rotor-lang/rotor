
#[test]
fn lex_let_stmt() {
    use rotor::lexer::{lex, TokenKind};

    let input = "let x = 42;";
    let lexed = lex(input);

    assert_eq!(lexed.tokens.len(), 5);
    assert_eq!(lexed.tokens[0].kind, TokenKind::Let);
    assert_eq!(lexed.tokens[1].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[2].kind, TokenKind::Equal);
    assert_eq!(lexed.tokens[3].kind, TokenKind::Integer);
    assert_eq!(lexed.tokens[4].kind, TokenKind::Semicolon);
}

#[test]
fn lex_function_call() {
    use rotor::lexer::{lex, TokenKind};

    let input = "foo(\"bar\");";
    let lexed = lex(input);

    assert_eq!(lexed.tokens.len(), 5);
    assert_eq!(lexed.tokens[0].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[1].kind, TokenKind::LParen);
    assert_eq!(lexed.tokens[2].kind, TokenKind::String);
    assert_eq!(lexed.tokens[3].kind, TokenKind::RParen);
    assert_eq!(lexed.tokens[4].kind, TokenKind::Semicolon);
}

#[test]
fn lex_operators_and_brackets() {
    use rotor::lexer::{lex, TokenKind};

    let input = "(a + b) - [c * d] / {e % f};";
    let lexed = lex(input);

    assert_eq!(lexed.tokens.len(), 18);
    assert_eq!(lexed.tokens[0].kind, TokenKind::LParen);
    assert_eq!(lexed.tokens[1].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[2].kind, TokenKind::Plus);
    assert_eq!(lexed.tokens[3].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[4].kind, TokenKind::RParen);
    assert_eq!(lexed.tokens[5].kind, TokenKind::Line);
    assert_eq!(lexed.tokens[6].kind, TokenKind::LSquare);
    assert_eq!(lexed.tokens[7].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[8].kind, TokenKind::Star);
    assert_eq!(lexed.tokens[9].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[10].kind, TokenKind::RSquare);
    assert_eq!(lexed.tokens[11].kind, TokenKind::Slash);
    assert_eq!(lexed.tokens[12].kind, TokenKind::LCurly);
    assert_eq!(lexed.tokens[13].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[14].kind, TokenKind::Modulus);
    assert_eq!(lexed.tokens[15].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[16].kind, TokenKind::RCurly);
    assert_eq!(lexed.tokens[17].kind, TokenKind::Semicolon);

}