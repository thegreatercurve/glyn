use crate::assert_lexer_eq;

#[test]
fn numbers() {
    assert_lexer_eq!("1", [Token::Int64("1")]);
    assert_lexer_eq!("322", [Token::Int64("322")]);
    assert_lexer_eq!("3.3", [Token::Float64("3.3")]);
    assert_lexer_eq!("44444.55556", [Token::Float64("44444.55556")]);
}
