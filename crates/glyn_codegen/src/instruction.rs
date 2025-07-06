use std::fmt::{Display, Formatter};

use crate::bytecode_generator::BytecodeProgram;

pub(crate) enum Instruction {
    // Binary operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    // Comparison operations
    StrictEqual,
    StrictNotEqual,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Unary operations
    Not,
    Plus,
    Minus,
    Increment,
    Decrement,

    // Bitwise operations
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    // Logical operations
    LogicalAnd,
    LogicalOr,

    // Value operations
    Const,
    True,
    False,
    Null,
    Undefined,

    // Control flow
    Jump,
    JumpIfTrue,
    JumpIfFalse,

    // Call stack operations
    SetLocal,
    GetLocal,
    Call,

    // Data stack operations
    Return,
    Pop,

    // Utility operations
    Print,
    Halt,

    InitializeReferencedBinding,
    ResolveBinding,
}

impl Instruction {
    pub(crate) fn n_operands(&self) -> usize {
        match self {
            Instruction::Add
            | Instruction::Subtract
            | Instruction::Multiply
            | Instruction::Divide
            | Instruction::Modulo
            | Instruction::Exponent
            | Instruction::StrictEqual
            | Instruction::StrictNotEqual
            | Instruction::Equal
            | Instruction::NotEqual
            | Instruction::LessThan
            | Instruction::LessThanOrEqual
            | Instruction::GreaterThan
            | Instruction::GreaterThanOrEqual
            | Instruction::Not
            | Instruction::Plus
            | Instruction::Minus
            | Instruction::Increment
            | Instruction::Decrement
            | Instruction::BitwiseAnd
            | Instruction::BitwiseOr
            | Instruction::BitwiseXor
            | Instruction::BitwiseShiftLeft
            | Instruction::BitwiseShiftRight
            | Instruction::LogicalAnd
            | Instruction::LogicalOr
            | Instruction::True
            | Instruction::False
            | Instruction::Null
            | Instruction::Undefined
            | Instruction::Return
            | Instruction::Pop
            | Instruction::Print
            | Instruction::Halt => 0,
            Instruction::Jump
            | Instruction::JumpIfTrue
            | Instruction::JumpIfFalse
            | Instruction::Const
            | Instruction::SetLocal
            | Instruction::GetLocal
            | Instruction::InitializeReferencedBinding
            | Instruction::ResolveBinding => 1,
            Instruction::Call => 2,
        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Instruction::Add,
            0x01 => Instruction::Subtract,
            0x02 => Instruction::Multiply,
            0x03 => Instruction::Divide,
            0x04 => Instruction::Modulo,
            0x05 => Instruction::Exponent,
            0x06 => Instruction::StrictEqual,
            0x07 => Instruction::StrictNotEqual,
            0x08 => Instruction::Equal,
            0x09 => Instruction::NotEqual,
            0x0A => Instruction::LessThan,
            0x0B => Instruction::LessThanOrEqual,
            0x0C => Instruction::GreaterThan,
            0x0D => Instruction::GreaterThanOrEqual,
            0x0E => Instruction::Not,
            0x0F => Instruction::Plus,
            0x10 => Instruction::Minus,
            0x11 => Instruction::Increment,
            0x12 => Instruction::Decrement,
            0x13 => Instruction::BitwiseAnd,
            0x14 => Instruction::BitwiseOr,
            0x15 => Instruction::BitwiseXor,
            0x16 => Instruction::BitwiseShiftLeft,
            0x17 => Instruction::BitwiseShiftRight,
            0x18 => Instruction::LogicalAnd,
            0x19 => Instruction::LogicalOr,
            0x1A => Instruction::Const,
            0x1B => Instruction::True,
            0x1C => Instruction::False,
            0x1D => Instruction::Null,
            0x1E => Instruction::Undefined,
            0x1F => Instruction::Jump,
            0x20 => Instruction::JumpIfTrue,
            0x21 => Instruction::JumpIfFalse,
            0x22 => Instruction::SetLocal,
            0x23 => Instruction::GetLocal,
            0x24 => Instruction::Call,
            0x25 => Instruction::Return,
            0x26 => Instruction::Pop,
            0x27 => Instruction::Print,
            0x28 => Instruction::InitializeReferencedBinding,
            0x29 => Instruction::ResolveBinding,
            0xFF => Instruction::Halt,
            _ => unreachable!("Unknown instruction: {}", value),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Add => 0x00,
            Instruction::Subtract => 0x01,
            Instruction::Multiply => 0x02,
            Instruction::Divide => 0x03,
            Instruction::Modulo => 0x04,
            Instruction::Exponent => 0x05,
            Instruction::StrictEqual => 0x06,
            Instruction::StrictNotEqual => 0x07,
            Instruction::Equal => 0x08,
            Instruction::NotEqual => 0x09,
            Instruction::LessThan => 0x0A,
            Instruction::LessThanOrEqual => 0x0B,
            Instruction::GreaterThan => 0x0C,
            Instruction::GreaterThanOrEqual => 0x0D,
            Instruction::Not => 0x0E,
            Instruction::Plus => 0x0F,
            Instruction::Minus => 0x10,
            Instruction::Increment => 0x11,
            Instruction::Decrement => 0x12,
            Instruction::BitwiseAnd => 0x13,
            Instruction::BitwiseOr => 0x14,
            Instruction::BitwiseXor => 0x15,
            Instruction::BitwiseShiftLeft => 0x16,
            Instruction::BitwiseShiftRight => 0x17,
            Instruction::LogicalAnd => 0x18,
            Instruction::LogicalOr => 0x19,
            Instruction::Const => 0x1A,
            Instruction::True => 0x1B,
            Instruction::False => 0x1C,
            Instruction::Null => 0x1D,
            Instruction::Undefined => 0x1E,
            Instruction::Jump => 0x1F,
            Instruction::JumpIfTrue => 0x20,
            Instruction::JumpIfFalse => 0x21,
            Instruction::SetLocal => 0x22,
            Instruction::GetLocal => 0x23,
            Instruction::Call => 0x24,
            Instruction::Return => 0x25,
            Instruction::Pop => 0x26,
            Instruction::Print => 0x27,
            Instruction::Halt => 0xFF,
            Instruction::InitializeReferencedBinding => 0x28,
            Instruction::ResolveBinding => 0x29,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add => f.pad("ADD"),
            Instruction::Subtract => f.pad("SUB"),
            Instruction::Multiply => f.pad("MUL"),
            Instruction::Divide => f.pad("DIV"),
            Instruction::Modulo => f.pad("MOD"),
            Instruction::Exponent => f.pad("EXP"),
            Instruction::StrictEqual => f.pad("STRICT_EQUAL"),
            Instruction::StrictNotEqual => f.pad("STRICT_NOT_EQUAL"),
            Instruction::Equal => f.pad("EQUAL"),
            Instruction::NotEqual => f.pad("NOT_EQUAL"),
            Instruction::LessThan => f.pad("LESS_THAN"),
            Instruction::LessThanOrEqual => f.pad("LESS_THAN_OR_EQUAL"),
            Instruction::GreaterThan => f.pad("GREATER_THAN"),
            Instruction::GreaterThanOrEqual => f.pad("GREATER_THAN_OR_EQUAL"),
            Instruction::Not => f.pad("NOT"),
            Instruction::Plus => f.pad("PLUS"),
            Instruction::Minus => f.pad("MINUS"),
            Instruction::Increment => f.pad("INC"),
            Instruction::Decrement => f.pad("DEC"),
            Instruction::BitwiseAnd => f.pad("BIT_AND"),
            Instruction::BitwiseOr => f.pad("BIT_OR"),
            Instruction::BitwiseXor => f.pad("BIT_XOR"),
            Instruction::BitwiseShiftLeft => f.pad("BIT_SHIFT_LEFT"),
            Instruction::BitwiseShiftRight => f.pad("BIT_SHIFT_RIGHT"),
            Instruction::LogicalAnd => f.pad("LOG_AND"),
            Instruction::LogicalOr => f.pad("LOG_OR"),
            Instruction::Const => f.pad("CONST"),
            Instruction::True => f.pad("TRUE"),
            Instruction::False => f.pad("FALSE"),
            Instruction::Null => f.pad("NULL"),
            Instruction::Undefined => f.pad("UNDEFINED"),
            Instruction::Jump => f.pad("JUMP"),
            Instruction::JumpIfTrue => f.pad("JUMP_IF_TRUE"),
            Instruction::JumpIfFalse => f.pad("JUMP_IF_FALSE"),
            Instruction::SetLocal => f.pad("SET_LOCAL"),
            Instruction::GetLocal => f.pad("GET_LOCAL"),
            Instruction::Call => f.pad("CALL"),
            Instruction::Return => f.pad("RETURN"),
            Instruction::Pop => f.pad("POP"),
            Instruction::Print => f.pad("PRINT"),
            Instruction::Halt => f.pad("HALT"),
            Instruction::InitializeReferencedBinding => f.pad("INITIALIZE_REFERENCED_BINDING"),
            Instruction::ResolveBinding => f.pad("RESOLVE_BINDING"),
        }
    }
}

// #[cfg(feature = "debug")]
pub(crate) struct Disassembler {
    program: BytecodeProgram,
}

// #[cfg(feature = "debug")]
impl Disassembler {
    pub(crate) fn new(program: BytecodeProgram) -> Self {
        Disassembler { program }
    }

    pub(crate) fn initial_bytecode(&self) -> String {
        let mut result = String::new();

        let mut ip = 0;

        let instructions = &self.program.instructions;

        while ip < instructions.len() {
            let formatted_instruction = self.disassemble_instruction(&mut ip, instructions);

            result.push_str(&formatted_instruction);

            result.push('\n');
        }

        result
    }

    fn disassemble_instruction(&self, ip: &mut usize, program: &[u8]) -> String {
        let instruction = Instruction::from(program[*ip]);

        let mut result = String::new();

        result.push_str(&format!("\t{:04} {:<15}", ip, instruction));

        *ip += 1;

        let n_operands = instruction.n_operands();

        for _ in 0..n_operands {
            let operand = program[*ip];

            result.push_str(&format!("{:<5}", operand));

            *ip += 1;
        }

        result
    }
}
