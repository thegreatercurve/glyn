use crate::value::{number::JSNumber, string::JSString, JSValue};

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Clone, Debug, PartialEq)]
pub enum JSObjectPropKey {
    String(JSString),
    Symbol(JSValue),
}

impl JSObjectPropKey {
    pub(crate) fn is_string(&self) -> bool {
        matches!(self, JSObjectPropKey::String(_))
    }

    pub(crate) fn is_symbol(&self) -> bool {
        matches!(self, JSObjectPropKey::Symbol(_))
    }

    /// An array index is an integer index n such that CanonicalNumericIndexString(n) returns
    /// an integral Number in the inclusive interval from +0𝔽 to 𝔽(2****32 - 2).
    /// https://262.ecma-international.org/15.0/#sec-object-type
    pub(crate) fn as_array_index(&self) -> Option<u32> {
        if let JSObjectPropKey::String(value) = self {
            if let Ok(JSNumber::UInt(number)) = JSNumber::try_from(value.clone()) {
                return Some(number);
            }
        }

        None
    }

    pub(crate) fn is_array_index(&self) -> bool {
        self.as_array_index().is_some()
    }
}

/// 6.2.6 The Property Descriptor Specification Type
/// https://262.ecma-international.org/15.0/#sec-property-descriptor-specification-type
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSObjectPropDescriptor {
    /// [[Value]]
    pub value: Option<JSValue>,

    /// [[Writable]]
    pub writable: Option<bool>,

    /// [[Get]]
    pub get: Option<JSValue>,

    /// [[Set]]
    pub set: Option<JSValue>,

    /// [[Enumerable]]
    pub enumerable: Option<bool>,

    /// [[Configurable]]
    pub configurable: Option<bool>,
}

impl JSObjectPropDescriptor {
    pub(crate) fn is_fully_populated(&self) -> bool {
        self.value.is_some()
            && self.writable.is_some()
            && self.get.is_some()
            && self.set.is_some()
            && self.enumerable.is_some()
            && self.configurable.is_some()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_none()
            && self.writable.is_none()
            && self.get.is_none()
            && self.set.is_none()
            && self.enumerable.is_none()
            && self.configurable.is_none()
    }
}

impl JSObjectPropDescriptor {
    /// 6.2.6.1 IsAccessorDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/#sec-property-descriptor-specification-type
    pub(crate) fn is_accessor_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Get]] field, return true.
        // 3. If Desc has a [[Set]] field, return true.
        // 4. Return false.
        self.get.is_some() || self.set.is_some()
    }

    /// 6.2.6.2 IsDataDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/#sec-isdatadescriptor
    pub(crate) fn is_data_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Value]] field, return true.
        // 3. If Desc has a [[Writable]] field, return true.
        // 4. Return false.
        self.value.is_some() || self.writable.is_some()
    }

    /// 6.2.6.3 IsGenericDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/#sec-isgenericdescriptor
    pub(crate) fn is_generic_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Value]] field, return true.
        // 3. If Desc has a [[Writable]] field, return true.
        // 4. Return false.
        self.value.is_some() || self.writable.is_some()
    }
}
