use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[repr(u8)]
pub(crate) enum Instruction {
    BinAdd,
    BinDivide,
    BinExponent,
    BinModulo,
    BinMultiply,
    BinSubtract,
    BitAnd,
    BitOr,
    BitShiftLeft,
    BitShiftRight,
    BitShiftRightUnsigned,
    BitXor,
    Call,
    Const,
    CreateMutableBinding,
    Decrement,
    Equal,
    False,
    GetLocal,
    GreaterThan,
    GreaterThanOrEqual,
    Halt,
    Increment,
    InitializeReferencedBinding,
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    Minus,
    Not,
    NotEqual,
    Null,
    Plus,
    Pop,
    Print,
    ResolveBinding,
    Return,
    StrictEqual,
    StrictNotEqual,
    True,
    Undefined,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        // Safety: The u8 values should be within the range of the Instruction enum.
        unsafe { std::mem::transmute(value) }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
