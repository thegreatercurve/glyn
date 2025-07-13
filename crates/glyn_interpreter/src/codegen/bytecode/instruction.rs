use std::fmt::{Display, Formatter};

#[repr(u8)]
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
    BitAnd,
    BitOr,
    BitXor,
    BitShiftLeft,
    BitShiftRight,
    BitShiftRightUnsigned,

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

    // Reference operations
    ResolveBinding,
    InitializeReferencedBinding,

    // Utility operations
    Print,
    Halt,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        // Safety: The u8 values should be within the range of the Instruction enum.
        unsafe { std::mem::transmute(value) }
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
            Instruction::BitAnd => f.pad("BIT_AND"),
            Instruction::BitOr => f.pad("BIT_OR"),
            Instruction::BitXor => f.pad("BIT_XOR"),
            Instruction::BitShiftLeft => f.pad("BIT_SHIFT_LEFT"),
            Instruction::BitShiftRight => f.pad("BIT_SHIFT_RIGHT"),
            Instruction::BitShiftRightUnsigned => f.pad("BIT_SHIFT_RIGHT_UNSIGNED"),
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
            Instruction::ResolveBinding => f.pad("RESOLVE_BINDING"),
            Instruction::InitializeReferencedBinding => f.pad("INITIALIZE_REFERENCED_BINDING"),
            Instruction::Print => f.pad("PRINT"),
            Instruction::Halt => f.pad("HALT"),
        }
    }
}
