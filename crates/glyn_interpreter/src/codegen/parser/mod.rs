mod expression;
mod imports_and_modules;
mod statement;

use std::iter::Peekable;

use crate::{
    codegen::{
        bytecode::generator::{BytecodeGenerator, FinalProgram},
        error::{CodeGenError, CodeGenResult},
    },
    lexer::{Lexer, Token},
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

    pub(crate) fn program(self) -> FinalProgram {
        self.bytecode.program()
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
}
