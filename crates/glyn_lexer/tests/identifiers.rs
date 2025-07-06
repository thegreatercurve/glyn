mod common;

#[test]
fn identifiers() {
    use glyn_lexer::Token;

    assert_lexer_eq!("helloWorld", [Token::Ident("helloWorld")]);
    assert_lexer_eq!("HelloWorld", [Token::Ident("HelloWorld")]);
}
