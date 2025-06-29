use crate::value::big_int::JSBigInt;

mod big_int;
mod comparison;
mod number;
mod object;
mod string;
mod symbol;

pub use number::JSNumber;
pub use object::JSObject;
pub use object::{make_basic_object, JSObjectPropDescriptor, JSObjectPropKey};
use safe_gc::Gc;
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
    Object(Gc<JSObject>),
}

impl From<Gc<JSObject>> for JSValue {
    fn from(object: Gc<JSObject>) -> Self {
        JSValue::Object(object)
    }
}

impl JSValue {
    pub(crate) fn as_number(&self) -> Option<&JSNumber> {
        match self {
            JSValue::Number(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_number_mut(&mut self) -> Option<&mut JSNumber> {
        match self {
            JSValue::Number(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn is_object(&self) -> bool {
        matches!(self, JSValue::Object(_))
    }

    pub(crate) fn as_object(&self) -> Option<Gc<JSObject>> {
        match self {
            JSValue::Object(object) => Some(*object),
            _ => None,
        }
    }

    pub(crate) fn as_object_mut(&mut self) -> Option<&mut Gc<JSObject>> {
        match self {
            JSValue::Object(object) => Some(object),
            _ => None,
        }
    }
}

