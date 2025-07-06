use glyn_execution_model::value::string::JSString;
use glyn_lexer::Token;

use crate::instruction::Instruction;

pub(crate) enum LiteralType {
    Null,
    Boolean(bool),
    Int64(f64),
}

#[derive(Debug)]
pub(crate) enum BytecodeGeneratorError {
    InvalidTokenToBinaryOpConversion,
    InvalidTokenToUnaryOpConversion,
    UnboundIdentifierReference,
}

pub(crate) type BytecodeGeneratorResult = Result<(), BytecodeGeneratorError>;

#[derive(Debug, Default)]
pub(crate) struct BytecodeProgram {
    pub(crate) constants: Vec<f64>,
    pub(crate) instructions: Vec<u8>,
}

#[derive(Debug, Default)]
pub(crate) struct BytecodeGenerator {
    identifiers: Vec<JSString>,
    program: BytecodeProgram,
}

impl BytecodeGenerator {
    pub(crate) fn program(self) -> BytecodeProgram {
        self.program
    }

    fn error(&self, error: BytecodeGeneratorError) -> BytecodeGeneratorResult {
        Err(error)
    }

    fn emit_instr(&mut self, byte: Instruction) {
        self.program.instructions.push(byte as u8);
    }

    fn emit_instr_one_arg(&mut self, instruction: Instruction, arg: u8) {
        self.emit_instr(instruction);

        self.program.instructions.push(arg);
    }

    fn add_constant(&mut self, value: f64) -> u8 {
        self.program.constants.push(value);

        (self.program.constants.len() - 1) as u8
    }

    fn add_identifier(&mut self, identifier: JSString) -> u8 {
        self.identifiers.push(identifier);

        (self.identifiers.len() - 1) as u8
    }

    pub(crate) fn compile_print(&mut self) {
        self.emit_instr(Instruction::Print);
    }

    pub(crate) fn compile_assigment_op(&mut self, _ops: &Token) {
        todo!()
    }

    pub(crate) fn compile_binary_op(&mut self, op_token: &Token) -> BytecodeGeneratorResult {
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
            Token::BitwiseAnd => Instruction::BitwiseAnd,
            Token::BitwiseOr => Instruction::BitwiseOr,
            Token::BitwiseXor => Instruction::BitwiseXor,
            Token::LeftShift => Instruction::BitwiseShiftLeft,
            Token::RightShift => Instruction::BitwiseShiftRight,
            Token::UnsignedRightShift => Instruction::BitwiseShiftRight,
            Token::LogicalAnd => Instruction::LogicalAnd,
            Token::LogicalOr => Instruction::LogicalOr,
            _ => return self.error(BytecodeGeneratorError::InvalidTokenToBinaryOpConversion),
        };

        self.emit_instr(instruction);

        Ok(())
    }

    pub(crate) fn compile_unary_op(&mut self, op_token: &Token) -> BytecodeGeneratorResult {
        let instruction = match op_token {
            Token::Plus => Instruction::Plus,
            Token::Minus => Instruction::Minus,
            Token::Not => Instruction::Not,
            _ => return self.error(BytecodeGeneratorError::InvalidTokenToUnaryOpConversion),
        };

        self.emit_instr(instruction);

        Ok(())
    }

    /// 13.2.3 Literals
    /// https://tc39.es/ecma262/#prod-Literal
    ///  Literal : Null
    ///  Literal : BooleanLiteral
    ///  Literal : NumericLiteral
    ///  Literal : StringLiteral
    pub(crate) fn compile_literal(&mut self, literal: &LiteralType) -> BytecodeGeneratorResult {
        match literal {
            LiteralType::Null => self.emit_instr(Instruction::Null),
            LiteralType::Boolean(value) => {
                self.emit_instr(if *value {
                    Instruction::True
                } else {
                    Instruction::False
                });
            }
            LiteralType::Int64(value) => {
                let index = self.add_constant(*value);

                self.emit_instr_one_arg(Instruction::Const, index);
            }
        };

        Ok(())
    }

    // Statements
    pub(crate) fn compile_get_let_variable(&mut self) -> BytecodeGeneratorResult {
        let index = 0;

        self.emit_instr_one_arg(Instruction::GetLocal, index as u8);

        Ok(())
    }

    /// 14.3.1 Let and Const Declarations
    /// https://262.ecma-international.org/15.0/#sec-let-and-const-declarations
    /// LexicalBinding : BindingIdentifier
    pub(crate) fn compile_let_declaration_without_initializer(
        &mut self,
        name: JSString,
    ) -> BytecodeGeneratorResult {
        // 1. Let lhs be ! ResolveBinding(StringValue of BindingIdentifier).
        let index = self.add_identifier(name);
        self.emit_instr_one_arg(Instruction::ResolveBinding, index);

        // 2. Perform ! InitializeReferencedBinding(lhs, undefined).
        self.emit_instr(Instruction::Undefined);
        self.emit_instr_one_arg(Instruction::InitializeReferencedBinding, index);

        // 3. Return empty.
        Ok(())
    }

    /// LexicalBinding : BindingIdentifier Initializer
    pub(crate) fn compile_let_declaration_with_initializer(
        &mut self,
        binding_id: JSString,
    ) -> BytecodeGeneratorResult {
        // 1. Let bindingId be StringValue of BindingIdentifier.
        let index = self.add_identifier(binding_id);

        // 2. Let lhs be ! ResolveBinding(bindingId).
        self.emit_instr_one_arg(Instruction::ResolveBinding, index);

        // 5. Perform ? InitializeReferencedBinding(lhs, Initializer).
        self.emit_instr_one_arg(Instruction::InitializeReferencedBinding, index);

        // 6. Return empty.
        Ok(())
    }
}
