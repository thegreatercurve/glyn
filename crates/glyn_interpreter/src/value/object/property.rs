use crate::{value::string::JSString, JSValue};

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Clone, Debug, PartialEq)]
pub enum JSObjectPropKey {
    String(JSString),
    Symbol(JSValue),
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
    /// 6.2.6.1 IsAccessorDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/index.html#sec-property-descriptor-specification-type
    pub(crate) fn is_accessor_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Get]] field, return true.
        // 3. If Desc has a [[Set]] field, return true.
        // 4. Return false.
        self.get.is_some() || self.set.is_some()
    }

    /// 6.2.6.2 IsDataDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/index.html#sec-isdatadescriptor
    pub(crate) fn is_data_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Value]] field, return true.
        // 3. If Desc has a [[Writable]] field, return true.
        // 4. Return false.
        self.value.is_some() || self.writable.is_some()
    }

    /// 6.2.6.3 IsGenericDescriptor ( Desc )
    /// https://262.ecma-international.org/15.0/index.html#sec-isgenericdescriptor
    pub(crate) fn is_generic_descriptor(&self) -> bool {
        // 1. If Desc is undefined, return false.
        // 2. If Desc has a [[Value]] field, return true.
        // 3. If Desc has a [[Writable]] field, return true.
        // 4. Return false.
        self.value.is_some() || self.writable.is_some()
    }
}
