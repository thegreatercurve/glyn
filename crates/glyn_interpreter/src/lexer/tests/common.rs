#[macro_export]
macro_rules! assert_lexer_eq {
    ($input: expr, $expected_tokens: expr) => {{
        use $crate::lexer::{Keyword, Lexer, Token};

        let mut lexer = Lexer::new($input);

        for expected in $expected_tokens {
            let result = lexer.next().unwrap();

            assert_eq!(expected, result);
        }
    }};
}
