use crate::assert_lexer_eq;

#[test]
fn identifiers() {
    assert_lexer_eq!("helloWorld", [Token::Ident("helloWorld")]);
    assert_lexer_eq!("HelloWorld", [Token::Ident("HelloWorld")]);
}
