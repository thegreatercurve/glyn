use crate::value::big_int::JSBigInt;

mod big_int;
mod comparison;
mod conversion;
mod number;
mod object;
mod string;
mod symbol;

pub(crate) use object::JSObjAddr;

pub use number::JSNumber;
pub use object::JSObject;
pub use object::{make_basic_object, JSObjectPropDescriptor, JSObjectPropKey};
pub use string::JSString;

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
    fn is_boolean(&self) -> bool {
        matches!(self, JSValue::Boolean(_))
    }

    fn as_boolean(&self) -> Option<&bool> {
        match self {
            JSValue::Boolean(value) => Some(value),
            _ => None,
        }
    }

    fn as_number(&self) -> Option<&JSNumber> {
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

    fn is_object(&self) -> bool {
        matches!(self, JSValue::Object(_))
    }

    fn as_object(&self) -> Option<JSObjAddr> {
        match self {
            JSValue::Object(object) => Some(*object),
            _ => None,
        }
    }
}
