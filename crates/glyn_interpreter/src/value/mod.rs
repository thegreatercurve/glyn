pub(crate) mod big_int;
pub(crate) mod number;
pub(crate) mod object;
pub(crate) mod string;
pub(crate) mod symbol;

use crate::value::{big_int::JSBigInt, number::JSNumber, object::JSObjAddr, string::JSString};

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
    Boolean(bool),
    String(JSString),
    Number(JSNumber),
    BigInt(JSBigInt),
    Symbol,
    Object(JSObjAddr),
}

impl JSValue {
    pub(crate) fn is_boolean(&self) -> bool {
        matches!(self, JSValue::Boolean(_))
    }

    pub(crate) fn as_boolean(&self) -> Option<&bool> {
        match self {
            JSValue::Boolean(value) => Some(value),
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
        JSValue::Boolean(value)
    }
}

impl From<JSObjAddr> for JSValue {
    fn from(value: JSObjAddr) -> Self {
        JSValue::Object(value)
    }
}
