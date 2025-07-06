mod common;

#[test]
fn terminals() {
    use glyn_lexer::Token;

    assert_lexer_eq!("(", [Token::LeftParen]);
    assert_lexer_eq!(")", [Token::RightParen]);
    assert_lexer_eq!("{{", [Token::LeftBrace]);
    assert_lexer_eq!("}", [Token::RightBrace]);
    assert_lexer_eq!(";", [Token::Semicolon]);
}
