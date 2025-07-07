pub(crate) enum CodeGenError {
    UnexpectedToken,
    InvalidInteger64Literal,
}

pub(crate) type CodeGenResult<T = ()> = Result<T, CodeGenError>;
