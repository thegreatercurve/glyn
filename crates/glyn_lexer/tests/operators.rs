mod common;

#[test]
fn operators() {
    assert_lexer_eq!("+", [Token::Plus]);
    assert_lexer_eq!("-", [Token::Minus]);
    assert_lexer_eq!("*", [Token::Multiply]);
    assert_lexer_eq!("/", [Token::Divide]);
}
