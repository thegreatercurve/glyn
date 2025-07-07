use glyn_lexer::{Keyword, Token};

use crate::{
    error::CodeGenError,
    parser::{CodeGenResult, Parser},
};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14 ECMAScript Language: Statements and Declarations
    // https://tc39.es/ecma262/#prod-Statement
    fn js_parse_statement(&mut self) -> CodeGenResult {
        let current_token = self.current_token.clone();

        match current_token {
            Token::Keyword(Keyword::Let)
                if self
                    .peek()
                    .is_some_and(|token| token.is_lexical_binding_start()) =>
            {
                self.js_parse_let_declaration()
            }
            Token::Keyword(Keyword::Print) => self.js_parse_print_statement(),
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

    // 14.3.1 Let and Const Declarations
    // https://tc39.es/ecma262/#prod-LexicalDeclaration
    fn js_parse_let_declaration(&mut self) -> CodeGenResult {
        self.expect_one_of(vec![
            Token::Keyword(Keyword::Let),
            Token::Keyword(Keyword::Const),
        ])?;

        // TODO Check that const is a valid initializer.
        self.js_parse_binding_list()?;

        self.optional(Token::Semicolon);

        Ok(())
    }

    // https://tc39.es/ecma262/#prod-BindingList
    pub(crate) fn js_parse_binding_list(&mut self) -> CodeGenResult {
        let mut declarations = vec![self.js_parse_lexical_binding()?];

        // TODO Handle when in an in context.
        while self.current_token == Token::Comma {
            self.expect(Token::Comma)?;

            declarations.push(self.js_parse_lexical_binding()?);
        }

        Ok(())
    }

    // https://tc39.es/ecma262/#prod-LexicalBinding
    fn js_parse_lexical_binding(&mut self) -> CodeGenResult {
        let binding_identifier = match self.current_token.clone() {
            token_kind if token_kind.is_binding_identifier() => self.js_parse_binding_identifier(),
            Token::LeftBrace => todo!(),
            Token::LeftBracket => todo!(),
            _ => self.error(CodeGenError::UnexpectedToken),
        }?;

        if self.current_token == Token::Assign {
            self.advance(); // Eat '=' token.

            self.js_parse_assignment_expression()?;

            self.bytecode
                .compile_let_declaration_with_initializer(binding_identifier)?;
        } else {
            self.bytecode
                .compile_let_declaration_without_initializer(binding_identifier)?;
        };

        Ok(())
    }
}
