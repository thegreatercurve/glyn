mod common;

#[test]
fn whitespace() {
    assert_lexer_eq!("1 + 2", [Token::Int64("1"), Token::Plus, Token::Int64("2")]);
    assert_lexer_eq!(
        "1\t\n  + 2",
        [Token::Int64("1"), Token::Plus, Token::Int64("2")]
    );
}
