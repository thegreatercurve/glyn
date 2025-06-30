use crate::JSValue;

/// 6.2.4 The Completion Record Specification Type
/// https://262.ecma-international.org/15.0/#sec-completion-record-specification-type
pub type CompletionRecord<T = NormalCompletion> = Result<T, ThrowCompletion>;

/// 6.2.4.1 NormalCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-normalcompletion
#[derive(Debug, PartialEq)]
pub enum NormalCompletion {
    Value(JSValue),
    Unused,
}

impl From<bool> for NormalCompletion {
    fn from(value: bool) -> Self {
        NormalCompletion::Value(JSValue::Boolean(value))
    }
}

/// 6.2.4.2 ThrowCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-throwcompletion    
#[derive(Debug)]
pub enum ThrowCompletion {
    Throw(JSValue),
}
