use crate::runtime::{normal_completion, CompletionRecord};
use crate::value::big_int::JSBigInt;
use crate::JSAgent;

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

impl JSValue {
    /// 7.2.3 IsCallable ( argument )
    /// https://262.ecma-international.org/15.0/#sec-iscallable
    pub(crate) fn is_callable(&self, agent: &JSAgent) -> bool {
        // If argument is not an Object, return false.
        let Some(object_ptr) = self.as_object() else {
            return false;
        };

        // 2. If argument has a [[Call]] internal method, return true.
        if agent.deref_object_ptr(object_ptr).methods.call.is_some() {
            return true;
        }

        // 3. Return false.
        false
    }

    /// 7.2.4 IsConstructor ( argument )
    /// https://262.ecma-international.org/15.0/#sec-isconstructor
    pub(crate) fn is_constructor(&self, agent: &JSAgent) -> bool {
        // If argument is not an Object, return false.
        let Some(object_ptr) = self.as_object() else {
            return false;
        };

        // 2. If argument has a [[Construct]] internal method, return true.
        if agent
            .deref_object_ptr(object_ptr)
            .methods
            .construct
            .is_some()
        {
            return true;
        }

        // 3. Return false.
        false
    }

    /// 7.2.7 IsPropertyKey ( argument )
    /// https://262.ecma-international.org/15.0/#sec-ispropertykey
    pub(crate) fn is_property_key(&self) -> bool {
        // 1. If Type(argument) is String, return true.
        // 2. If Type(argument) is Symbol, return false.
        // 3. Return false.
        matches!(self, JSValue::String(_) | JSValue::Symbol)
    }
}
