use crate::{
    runtime::completion::{throw_completion, ThrowCompletion},
    value::string::JSString,
    value::JSValue,
};

/// 6.1.8 The BigInt Type
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-bigint-type
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct JSBigInt;

impl JSBigInt {
    pub(crate) fn is_zero(&self) -> bool {
        false
    }
}

impl JSBigInt {
    pub(crate) fn to_string(&self, radix: u32) -> JSString {
        todo!()
    }
}

impl TryFrom<JSValue> for JSBigInt {
    type Error = ThrowCompletion;

    fn try_from(value: JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::BigInt(value) => Ok(value),
            _ => throw_completion("Expected a JSValue::BigInt for conversion to JSBigInt"),
        }
    }
}

impl TryFrom<&JSValue> for JSBigInt {
    type Error = ThrowCompletion;

    fn try_from(value: &JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::BigInt(value) => Ok(value.clone()),
            _ => throw_completion("Expected a JSValue::BigInt for conversion to JSBigInt"),
        }
    }
}
