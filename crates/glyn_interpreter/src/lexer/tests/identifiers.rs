mod common;

#[test]
fn identifiers() {
    assert_lexer_eq!("helloWorld", [Token::Ident("helloWorld")]);
    assert_lexer_eq!("HelloWorld", [Token::Ident("HelloWorld")]);
}
