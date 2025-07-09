use crate::value::JSValue;

/// 6.2.4 The Completion Record Specification Type
/// https://262.ecma-international.org/16.0/#sec-completion-record-specification-type
pub(crate) type CompletionRecord<T = ()> = Result<T, ThrowCompletion>;

/// 6.2.4.1 NormalCompletion ( value )
/// https://262.ecma-international.org/16.0/#sec-normalcompletion
#[derive(Debug, PartialEq)]
pub(crate) enum NormalCompletion {
    Bool(bool),
    Value(JSValue),
    Unused,
}

impl From<JSValue> for NormalCompletion {
    fn from(value: JSValue) -> Self {
        match value {
            JSValue::Undefined => NormalCompletion::Value(JSValue::Undefined),
            JSValue::Null => NormalCompletion::Value(JSValue::Null),
            JSValue::Bool(b) => NormalCompletion::Value(JSValue::Bool(b)),
            JSValue::Number(n) => NormalCompletion::Value(JSValue::Number(n)),
            JSValue::String(s) => NormalCompletion::Value(JSValue::String(s)),
            JSValue::Object(o) => NormalCompletion::Value(JSValue::Object(o)),
            JSValue::BigInt(b) => NormalCompletion::Value(JSValue::BigInt(b)),
            JSValue::Symbol(s) => NormalCompletion::Value(JSValue::Symbol(s)),
        }
    }
}

/// 6.2.4.2 ThrowCompletion ( value )
/// https://262.ecma-international.org/16.0/#sec-throwcompletion    
#[derive(Debug)]
pub(crate) enum ThrowCompletion {
    Throw(JSValue),
}
