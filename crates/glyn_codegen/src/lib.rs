pub(crate) mod bytecode_generator;
pub(crate) mod error;
pub(crate) mod instruction;
pub(crate) mod parser;

use crate::error::CodeGenError;

pub(crate) type CodeGenResult<T = ()> = Result<T, CodeGenError>;
