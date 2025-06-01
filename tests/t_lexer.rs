use rotor::globals::lexer::{lex, TokenKind};

#[test]
fn test_lex_simple() {
    let src = "let x = 5;";
    let lexed = lex(src);

    assert!(lexed.is_working());
    assert_eq!(lexed.tokens[0].get_debug_info().contains("Let"), true);
}

#[test]
fn test_parentheses() {
    let src = "(a + b) {c - d} [e * f]";
    let lexed = lex(src);

    assert!(lexed.is_working());
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::LParen));
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::RParen));
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::LCurly));
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::RCurly));
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::LSquare));
    assert!(lexed.tokens.iter().any(|t| t.kind == TokenKind::RSquare));
}