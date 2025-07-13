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
            Token::Keyword(Keyword::Let)
                if peek_token.is_some_and(|token| token.is_lexical_binding_start()) =>
            {
                self.js_parse_let_declaration()
            }
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

    /// 14.3.1 Let and Const Declarations
    /// https://262.ecma-international.org/16.0/#prod-LexicalBinding
    fn js_parse_let_declaration(&mut self) -> CodeGenResult {
        self.expect(Token::Keyword(Keyword::Let))?;

        let binding_identifier = match self.current_token.clone() {
            token_kind if token_kind.is_binding_identifier() => self.js_parse_binding_identifier(),
            Token::LeftBrace => todo!(),
            Token::LeftBracket => todo!(),
            _ => self.error(CodeGenError::UnexpectedToken),
        }?;

        let has_initializer = self.current_token == Token::Assign;

        if has_initializer {
            self.advance(); // Eat '=' token.

            self.js_parse_assignment_expression()?;
        }

        self.bytecode
            .let_declaration(binding_identifier, has_initializer)?;

        Ok(())
    }
}
