use std::fmt::Display;

pub(crate) enum CodeGenError {
    UnexpectedToken,
    InvalidInteger64Literal,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeGenError::UnexpectedToken => write!(f, "Unexpected token"),
            CodeGenError::InvalidInteger64Literal => write!(f, "Invalid integer64 literal"),
        }
    }
}

pub(crate) type CodeGenResult<T = ()> = Result<T, CodeGenError>;
