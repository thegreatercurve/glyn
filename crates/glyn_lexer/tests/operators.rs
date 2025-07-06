mod common;

#[test]
fn operators() {
    use glyn_lexer::Token;

    assert_lexer_eq!("+", [Token::Plus]);
    assert_lexer_eq!("-", [Token::Minus]);
    assert_lexer_eq!("*", [Token::Multiply]);
    assert_lexer_eq!("/", [Token::Divide]);
}
