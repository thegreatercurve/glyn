use crate::{
    codegen::{
        bytecode::instruction::Instruction,
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
pub(crate) struct ExecutableProgram {
    pub(crate) instructions: Vec<u8>,
    pub(crate) constants: Vec<JSValue>,
    pub(crate) identifiers: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct BytecodeGenerator {
    instructions: Vec<u8>,
    constants: Vec<JSValue>,
    identifiers: Vec<String>,
}

impl BytecodeGenerator {
    pub(crate) fn program(self) -> ExecutableProgram {
        ExecutableProgram {
            instructions: self.instructions,
            constants: self.constants,
            identifiers: self.identifiers,
        }
    }

    fn push(&mut self, instruction: u8) {
        self.instructions.push(instruction);
    }

    pub(crate) fn emit_identifier(&mut self, identifier: JSString) -> u8 {
        self.identifiers.push(identifier.0);

        (self.identifiers.len() - 1) as u8
    }

    pub(crate) fn emit_constant(&mut self, value: JSValue) {
        self.constants.push(value);

        self.push(Instruction::Const as u8);
        self.push(self.constants.len() as u8 - 1);
    }

    pub(crate) fn emit_null(&mut self) {
        self.push(Instruction::Null as u8);
    }

    pub(crate) fn emit_undefined(&mut self) {
        self.push(Instruction::Undefined as u8);
    }

    pub(crate) fn emit_boolean(&mut self, value: bool) {
        self.push(if value {
            Instruction::True
        } else {
            Instruction::False
        } as u8);
    }

    pub(crate) fn emit_unary_exp(&mut self, instruction: Instruction) {
        self.push(instruction as u8);
    }

    pub(crate) fn emit_binary_exp(&mut self, instruction: Instruction) {
        self.push(instruction as u8);
    }

    pub(crate) fn emit_resolve_binding(&mut self, identifier_index: u8) {
        self.push(Instruction::ResolveBinding as u8);
        self.push(identifier_index);
    }

    pub(crate) fn emit_initialize_referenced_binding(&mut self) {
        self.push(Instruction::InitializeReferencedBinding as u8);
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

        self.emit_binary_exp(instruction);

        Ok(())
    }

    pub(crate) fn generate_unary_exp(&mut self, op_token: &Token) -> CodeGenResult {
        let instruction = match op_token {
            Token::Plus => Instruction::Plus,
            Token::Minus => Instruction::Minus,
            Token::Not => Instruction::Not,
            _ => return Err(CodeGenError::UnexpectedToken),
        };

        self.emit_unary_exp(instruction);

        Ok(())
    }

    /// 13.2.3 Literals
    /// https://262.ecma-international.org/16.0/#prod-Literal
    ///  Literal : Null
    ///  Literal : BooleanLiteral
    ///  Literal : NumericLiteral
    ///  Literal : StringLiteral
    pub(crate) fn generate_literal(&mut self, literal: &LiteralType) -> CodeGenResult {
        match literal {
            LiteralType::Null => self.emit_null(),
            LiteralType::Boolean(value) => self.emit_boolean(*value),
            LiteralType::Int64(value) => self.emit_constant(JSValue::from(*value)),
            LiteralType::String(value) => self.emit_constant(JSValue::from(value.clone())),
        };

        Ok(())
    }

    /// 14.3.1 Let and Const Declarations
    /// https://262.ecma-international.org/16.0/#sec-let-and-const-declarations
    /// LexicalBinding : BindingIdentifier
    /// LexicalBinding : BindingIdentifier Initializer
    pub(crate) fn let_declaration(
        &mut self,
        binding_id: JSString,
        has_initializer: bool,
    ) -> CodeGenResult {
        if has_initializer {
            todo!()
        } else {
            // 1. Let lhs be ! ResolveBinding(StringValue of BindingIdentifier).
            let binding_id = self.emit_identifier(binding_id);
            self.emit_resolve_binding(binding_id);

            // 2. Perform ! InitializeReferencedBinding(lhs, undefined).
            self.emit_undefined();
            self.emit_initialize_referenced_binding();
        }

        Ok(())
    }
}
