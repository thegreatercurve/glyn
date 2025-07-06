mod common;

#[test]
fn strings() {
    assert_lexer_eq!(r#""Hello world""#, [Token::String(r#""Hello world""#)]);
    assert_lexer_eq!(r#""""#, [Token::String(r#""""#)]);
    assert_lexer_eq!(
        r#""Hello ✅🙂✅ world""#,
        [Token::String(r#""Hello ✅🙂✅ world""#)]
    );
    assert_lexer_eq!(
        r#""function if else""#,
        [Token::String(r#""function if else""#)]
    );
}
