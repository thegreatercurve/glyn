use crate::{
    codegen::{
        error::CodeGenError,
        parser::{CodeGenResult, Parser},
    },
    lexer::{Keyword, Token},
};

/// 14 ECMAScript Language: Statements and Declarations
/// https://262.ecma-international.org/16.0/#prod-Statement
impl<'a> Parser<'a> {
    /// 14 ECMAScript Language: Statements and Declarations
    /// https://262.ecma-international.org/16.0/#prod-Statement
    fn js_parse_statement(&mut self) -> CodeGenResult {
        let current_token = self.current_token.clone();
        let peek_token = self.peek();

        match current_token {
            // Token::Keyword(Keyword::Let)
            //     if peek_token.is_some_and(|token| token.is_lexical_binding_start()) =>
            // {
            //     self.js_parse_let_declaration()
            // }
            _ => self.js_parse_expression(),
        }?;

        self.optional(Token::Semicolon);

        Ok(())
    }

    /// 14.2 Block
    /// https://262.ecma-international.org/16.0/#prod-StatementList
    pub(crate) fn js_parse_statement_list(&mut self) -> CodeGenResult {
        while !self.is_eof() {
            self.js_parse_statement()?;
        }

        Ok(())
    }
}
