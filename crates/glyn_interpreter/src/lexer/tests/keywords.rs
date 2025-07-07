mod common;

#[test]
fn keywords() {
    assert_lexer_eq!("let", [Token::Keyword(Keyword::Let)]);
    assert_lexer_eq!("if", [Token::Keyword(Keyword::If)]);
    assert_lexer_eq!("else", [Token::Keyword(Keyword::Else)]);
    assert_lexer_eq!("return", [Token::Keyword(Keyword::Return)]);
    assert_lexer_eq!("function", [Token::Keyword(Keyword::Function)]);
}
