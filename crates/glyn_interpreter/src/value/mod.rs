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
    pub(crate) fn to_boolean(&self) -> bool {
        match self {
            JSValue::Boolean(value) => *value,
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_boolean_mut(&mut self) -> &mut bool {
        match self {
            JSValue::Boolean(value) => value,
            _ => unreachable!(),
        }
    }

    pub(crate) fn as_number(&self) -> &JSNumber {
        match self {
            JSValue::Number(value) => value,
            _ => unreachable!(),
        }
    }

    pub(crate) fn as_number_mut(&mut self) -> &mut JSNumber {
        match self {
            JSValue::Number(value) => value,
            _ => unreachable!(),
        }
    }

    pub(crate) fn is_object(&self) -> bool {
        matches!(self, JSValue::Object(_))
    }

    pub(crate) fn to_object(&self) -> Gc<JSObject> {
        match self {
            JSValue::Object(object) => *object,
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_object_mut(&mut self) -> &mut Gc<JSObject> {
        match self {
            JSValue::Object(object) => object,
            _ => unreachable!(),
        }
    }
}

impl JSValue {
    /// 7.2.7 IsPropertyKey ( argument )
    /// https://262.ecma-international.org/15.0/#sec-ispropertykey
    pub(crate) fn is_property_key(&self) -> bool {
        // 1. If Type(argument) is String, return true.
        // 2. If Type(argument) is Symbol, return false.
        // 3. Return false.
        matches!(self, JSValue::String(_) | JSValue::Symbol)
    }
}
