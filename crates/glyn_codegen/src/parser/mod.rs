mod expression;
mod imports_and_modules;
mod statement;

use std::iter::Peekable;

use glyn_lexer::{Keyword, Lexer, Token};

use crate::{
    bytecode::generator::{BytecodeGenerator, BytecodeProgram},
    error::{CodeGenError, CodeGenResult},
};

pub(crate) struct Parser<'a> {
    bytecode: BytecodeGenerator,
    lexer: Peekable<Lexer<'a>>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: Lexer<'a>) -> Self {
        let mut lexer = lexer.peekable();

        let current_token = lexer.next().unwrap_or(Token::Illegal);

        Self {
            current_token,
            lexer,
            bytecode: BytecodeGenerator::default(),
        }
    }

    fn error<T>(&self, error: CodeGenError) -> CodeGenResult<T> {
        Err(error)
    }

    fn advance(&mut self) -> &Token {
        self.current_token = self.lexer.next().unwrap_or(Token::Eof);

        &self.current_token
    }

    pub(crate) fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    pub(crate) fn print_statement(&mut self) -> CodeGenResult {
        self.bytecode.compile_print();

        Ok(())
    }

    fn optional(&mut self, expected_token: Token) {
        if self.current_token == expected_token {
            self.advance();
        }
    }

    fn expect(&mut self, expected_token: Token) -> CodeGenResult {
        if self.current_token != expected_token {
            return self.error(CodeGenError::UnexpectedToken);
        }

        self.advance();

        Ok(())
    }

    fn expect_one_of(&mut self, expected_tokens: Vec<Token>) -> CodeGenResult {
        if !expected_tokens.contains(&self.current_token) {
            return self.error(CodeGenError::UnexpectedToken);
        }

        self.advance();

        Ok(())
    }

    fn is_eof(&self) -> bool {
        self.current_token == Token::Eof
    }

    fn js_parse_print_statement(&mut self) -> CodeGenResult {
        self.expect(Token::Keyword(Keyword::Print))?;

        self.expect(Token::LeftParen)?;

        self.js_parse_expression()?;

        self.expect(Token::RightParen)?;

        self.optional(Token::Semicolon);

        self.print_statement()?;

        Ok(())
    }
}

/// 11.1.6 Static Semantics: ParseText ( sourceText, goalSymbol )
/// https://262.ecma-international.org/15.0/#sec-parsetext
pub(crate) fn parse_text(source_text: &str) -> BytecodeProgram {
    // 1. Attempt to parse sourceText using goalSymbol as the goal symbol, and analyse the parse result for any early error conditions. Parsing and early error detection may be interleaved in an implementation-defined manner.
    let lexer = Lexer::new(source_text);
    let parser = Parser::new(lexer);

    // 2. If the parse succeeded and no early errors were found, return the Parse Node (an instance of goalSymbol) at the root of the parse tree resulting from the parse.
    // 3. Otherwise, return a List of one or more SyntaxError objects representing the parsing errors and/or early errors. If more than one parsing error or early error is present, the number and ordering of error objects in the list is implementation-defined, but at least one must be present.

    parser.bytecode.program()
}
