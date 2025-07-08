use crate::{
    codegen::{
        bytecode::{emitter::Emitter, instruction::Instruction},
        error::{CodeGenError, CodeGenResult},
    },
    lexer::Token,
    value::{string::JSString, JSValue},
};

pub(crate) enum LiteralType {
    Null,
    Boolean(bool),
    Int64(f64),
    String(String),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FinalProgram {
    pub(crate) instructions: Vec<u8>,
    pub(crate) constants: Vec<JSValue>,
}

#[derive(Debug, Default)]
pub(crate) struct BytecodeGenerator {
    emit: Emitter,
    identifiers: Vec<JSString>,
}

impl BytecodeGenerator {
    pub(crate) fn program(self) -> FinalProgram {
        self.emit.program()
    }

    fn add_identifier(&mut self, identifier: JSString) -> u8 {
        self.identifiers.push(identifier);

        (self.identifiers.len() - 1) as u8
    }

    pub(crate) fn generate_binary_exp(&mut self, op_token: &Token) -> CodeGenResult {
        let instruction = match op_token {
            Token::Plus => Instruction::Add,
            Token::Minus => Instruction::Subtract,
            Token::Multiply => Instruction::Multiply,
            Token::Divide => Instruction::Divide,
            Token::Exponent => Instruction::Exponent,
            Token::Modulo => Instruction::Modulo,
            Token::Equal => Instruction::Equal,
            Token::NotEqual => Instruction::NotEqual,
            Token::StrictEqual => Instruction::StrictEqual,
            Token::StrictNotEqual => Instruction::StrictNotEqual,
            Token::LessThan => Instruction::LessThan,
            Token::LessThanEqual => Instruction::LessThanOrEqual,
            Token::GreaterThan => Instruction::GreaterThan,
            Token::GreaterThanEqual => Instruction::GreaterThanOrEqual,
            Token::BitAnd => Instruction::BitAnd,
            Token::BitOr => Instruction::BitOr,
            Token::BitXor => Instruction::BitXor,
            Token::LeftShift => Instruction::BitShiftLeft,
            Token::RightShift => Instruction::BitShiftRight,
            Token::UnsignedRightShift => Instruction::BitShiftRight,
            Token::LogicalAnd => Instruction::LogicalAnd,
            Token::LogicalOr => Instruction::LogicalOr,
            _ => return Err(CodeGenError::UnexpectedToken),
        };

        self.emit.binary_exp(instruction);

        Ok(())
    }

    pub(crate) fn generate_unary_exp(&mut self, op_token: &Token) -> CodeGenResult {
        let instruction = match op_token {
            Token::Plus => Instruction::Plus,
            Token::Minus => Instruction::Minus,
            Token::Not => Instruction::Not,
            _ => return Err(CodeGenError::UnexpectedToken),
        };

        self.emit.unary_exp(instruction);

        Ok(())
    }

    /// 13.2.3 Literals
    /// https://tc39.es/ecma262/#prod-Literal
    ///  Literal : Null
    ///  Literal : BooleanLiteral
    ///  Literal : NumericLiteral
    ///  Literal : StringLiteral
    pub(crate) fn generate_literal(&mut self, literal: &LiteralType) -> CodeGenResult {
        match literal {
            LiteralType::Null => self.emit.null(),
            LiteralType::Boolean(value) => self.emit.boolean(*value),
            LiteralType::Int64(value) => self.emit.constant(JSValue::from(*value)),
            LiteralType::String(value) => self.emit.constant(JSValue::from(value.clone())),
        };

        Ok(())
    }
}
