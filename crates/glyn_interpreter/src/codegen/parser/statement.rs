use crate::{
    codegen::parser::{CodeGenResult, Parser},
    lexer::Token,
};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14 ECMAScript Language: Statements and Declarations
    // https://tc39.es/ecma262/#prod-Statement
    fn js_parse_statement(&mut self) -> CodeGenResult {
        let current_token = self.current_token.clone();

        match current_token {
            // Token::Keyword(Keyword::Let)
            //     if self
            //         .peek()
            //         .is_some_and(|token| token.is_lexical_binding_start()) =>
            // {
            //     self.js_parse_let_declaration()
            // }
            _ => self.js_parse_expression(),
        }?;

        self.optional(Token::Semicolon);

        Ok(())
    }

    // 14.2 Block
    // https://tc39.es/ecma262/#prod-StatementList
    pub(crate) fn js_parse_statement_list(&mut self) -> CodeGenResult {
        while !self.is_eof() {
            self.js_parse_statement()?;
        }

        Ok(())
    }
}
