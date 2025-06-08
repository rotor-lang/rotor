
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

    assert_eq!(lexed.tokens.len(), 13);
    assert_eq!(lexed.tokens[0].kind, TokenKind::LParen);
    assert_eq!(lexed.tokens[1].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[2].kind, TokenKind::Plus);
    assert_eq!(lexed.tokens[3].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[4].kind, TokenKind::RParen);
    assert_eq!(lexed.tokens[5].kind, TokenKind::Minus);
    assert_eq!(lexed.tokens[6].kind, TokenKind::LSquare);
    assert_eq!(lexed.tokens[7].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[8].kind, TokenKind::Multiply);
    assert_eq!(lexed.tokens[9].kind, TokenKind::Identifier);
    assert_eq!(lexed.tokens[10].kind, TokenKind::RSquare);
    assert_eq!(lexed.tokens[11].kind, TokenKind::Divide);
    assert_eq!(lexed.tokens[12].kind, TokenKind::Semicolon);
    
}