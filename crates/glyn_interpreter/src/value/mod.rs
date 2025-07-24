use crate::value::big_int::JSBigInt;
use crate::value::number::JSNumber;
use crate::value::object::ObjectAddr;
use crate::value::string::JSString;
use crate::value::symbol::JSSymbol;

pub(crate) mod big_int;
pub(crate) mod number;
pub(crate) mod object;
pub(crate) mod string;
pub(crate) mod symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum JSValue {
    /// 6.1.1 The Undefined Type
    /// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-undefined-type
    Undefined,
    /// 6.1.2 The Null Type
    /// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-null-type
    Null,
    /// 6.1.3 The Boolean Type
    /// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-boolean-type
    Bool(bool),
    String(JSString),
    Number(JSNumber),
    BigInt(JSBigInt),
    Symbol(JSSymbol),
    Object(ObjectAddr),
}

impl JSValue {
    pub(crate) fn is_undefined(&self) -> bool {
        self == &JSValue::Undefined
    }

    pub(crate) fn is_null(&self) -> bool {
        self == &JSValue::Null
    }

    pub(crate) fn is_boolean(&self) -> bool {
        matches!(self, JSValue::Bool(_))
    }

    pub(crate) fn is_string(&self) -> bool {
        matches!(self, JSValue::String(_))
    }

    pub(crate) fn is_number(&self) -> bool {
        matches!(self, JSValue::Number(_))
    }

    pub(crate) fn is_big_int(&self) -> bool {
        matches!(self, JSValue::BigInt(_))
    }

    pub(crate) fn is_object(&self) -> bool {
        matches!(self, JSValue::Object(_))
    }

    pub(crate) fn is_symbol(&self) -> bool {
        matches!(self, JSValue::Symbol(_))
    }
}

impl JSValue {
    pub(crate) fn is_nan(&self) -> bool {
        JSNumber::try_from(self).is_ok_and(|n| n.is_nan())
    }

    pub(crate) fn is_pos_infinite(&self) -> bool {
        JSNumber::try_from(self).is_ok_and(|n| n.is_pos_infinite())
    }

    pub(crate) fn is_neg_infinite(&self) -> bool {
        JSNumber::try_from(self).is_ok_and(|n| n.is_neg_infinite())
    }

    pub(crate) fn is_finite(&self) -> bool {
        JSNumber::try_from(self).is_ok_and(|n| n.is_finite())
    }
}

impl From<bool> for JSValue {
    fn from(value: bool) -> Self {
        JSValue::Bool(value)
    }
}

impl From<f64> for JSValue {
    fn from(value: f64) -> Self {
        JSValue::Number(JSNumber(value))
    }
}

impl From<u32> for JSValue {
    fn from(value: u32) -> Self {
        JSValue::Number(JSNumber(value as f64))
    }
}

impl From<i32> for JSValue {
    fn from(value: i32) -> Self {
        JSValue::Number(JSNumber(value as f64))
    }
}

impl From<JSNumber> for JSValue {
    fn from(value: JSNumber) -> Self {
        JSValue::Number(value)
    }
}

impl From<String> for JSValue {
    fn from(value: String) -> Self {
        JSValue::String(JSString::from(value))
    }
}

impl From<JSString> for JSValue {
    fn from(value: JSString) -> Self {
        JSValue::String(value)
    }
}

impl From<ObjectAddr> for JSValue {
    fn from(value: ObjectAddr) -> Self {
        JSValue::Object(value)
    }
}

impl From<&ObjectAddr> for JSValue {
    fn from(value: &ObjectAddr) -> Self {
        JSValue::Object(value.clone())
    }
}
