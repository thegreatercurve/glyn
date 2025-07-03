pub(crate) mod big_int;
pub(crate) mod number;
pub(crate) mod object;
pub(crate) mod string;
pub(crate) mod symbol;

use crate::value::{
    big_int::JSBigInt, number::JSNumber, object::JSObjAddr, string::JSString, symbol::JSSymbol,
};

#[derive(Clone, Debug, PartialEq)]
pub enum JSValue {
    /// 6.1.1 The Undefined Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-undefined-type
    Undefined,
    /// 6.1.2 The Null Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-null-type
    Null,
    /// 6.1.3 The Boolean Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-boolean-type
    Bool(bool),
    String(JSString),
    Number(JSNumber),
    BigInt(JSBigInt),
    Symbol(JSSymbol),
    Object(JSObjAddr),
}

impl JSValue {
    pub(crate) fn is_boolean(&self) -> bool {
        matches!(self, JSValue::Bool(_))
    }

    pub(crate) fn as_boolean(&self) -> Option<&bool> {
        match self {
            JSValue::Bool(value) => Some(value),
            _ => None,
        }
    }

    fn is_number(&self) -> bool {
        matches!(self, JSValue::Number(_))
    }

    pub(crate) fn as_number(&self) -> Option<&JSNumber> {
        match self {
            JSValue::Number(value) => Some(value),
            _ => None,
        }
    }

    fn as_number_mut(&mut self) -> Option<&mut JSNumber> {
        match self {
            JSValue::Number(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn is_big_int(&self) -> bool {
        matches!(self, JSValue::BigInt(_))
    }

    pub(crate) fn as_big_int(&self) -> Option<&JSBigInt> {
        match self {
            JSValue::BigInt(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_big_int_mut(&mut self) -> Option<&mut JSBigInt> {
        match self {
            JSValue::BigInt(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn is_object(&self) -> bool {
        matches!(self, JSValue::Object(_))
    }

    pub(crate) fn as_object(&self) -> Option<JSObjAddr> {
        match self {
            JSValue::Object(object) => Some(*object),
            _ => None,
        }
    }
}

impl From<bool> for JSValue {
    fn from(value: bool) -> Self {
        JSValue::Bool(value)
    }
}

impl From<f64> for JSValue {
    fn from(value: f64) -> Self {
        JSValue::Number(JSNumber::Float(value))
    }
}

impl From<u32> for JSValue {
    fn from(value: u32) -> Self {
        JSValue::Number(JSNumber::UInt(value))
    }
}

impl From<i32> for JSValue {
    fn from(value: i32) -> Self {
        JSValue::Number(JSNumber::Int(value))
    }
}

impl From<JSString> for JSValue {
    fn from(value: JSString) -> Self {
        JSValue::String(value)
    }
}

impl From<JSObjAddr> for JSValue {
    fn from(value: JSObjAddr) -> Self {
        JSValue::Object(value)
    }
}
