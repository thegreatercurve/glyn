mod internal_slots;
mod operations;
mod ordinary;
mod property;

use crate::{
    runtime::CompletionRecord, value::object::internal_slots::ObjectInternalSlots, JSAgent, JSValue,
};

pub use operations::make_basic_object;
pub use property::{JSObjectPropDescriptor, JSObjectPropKey};

/// Essential Internal Methods
/// https://262.ecma-international.org/15.0/index.html#table-essential-internal-methods
#[derive(Debug, PartialEq)]
pub struct JSObjectInternalMethods {
    /// [[GetPrototypeOf]]
    pub get_prototype_of: fn(agent: &JSAgent, object: &JSObject) -> Option<*mut JSObject>,
    /// [[SetPrototypeOf]]
    pub set_prototype_of: fn(object: &mut JSObject, prototype: Option<*mut JSObject>) -> bool,
    /// [[IsExtensible]]
    pub is_extensible: fn(object: &JSObject) -> bool,
    /// [[PreventExtensions]]
    pub prevent_extensions: fn(object: &mut JSObject) -> bool,
    /// [[GetOwnProperty]]
    pub get_own_property:
        fn(object: &JSObject, key: &JSObjectPropKey) -> Option<JSObjectPropDescriptor>,
    /// [[DefineOwnProperty]]
    pub define_own_property: fn(
        object: &mut JSObject,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> bool,
    /// [[HasProperty]]
    pub has_property: fn(agent: &JSAgent, object: &JSObject, key: &JSObjectPropKey) -> bool,
    /// [[Get]]
    pub get: fn(agent: &JSAgent, object: &JSObject, key: &JSObjectPropKey) -> CompletionRecord,
    /// [[Set]]
    pub set: fn(
        agent: &JSAgent,
        object: &mut JSObject,
        key: JSObjectPropKey,
        value: JSValue,
    ) -> CompletionRecord,
    /// [[Delete]]
    pub delete:
        fn(agent: &JSAgent, object: &mut JSObject, key: &JSObjectPropKey) -> CompletionRecord,
    /// [[OwnPropertyKeys]]
    pub own_property_keys: fn(agent: &JSAgent, object: &JSObject) -> Vec<JSObjectPropKey>,
}

struct PropertyIndex(usize);

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Clone, Debug, PartialEq)]
pub struct JSObject {
    pub methods: &'static JSObjectInternalMethods,
    pub slots: ObjectInternalSlots,
    pub keys: Vec<JSObjectPropKey>,
    pub values: Vec<JSObjectPropDescriptor>,
}

impl JSObject {
    /// All ordinary objects have an internal slot called [[Prototype]].
    pub(crate) fn ordinary_prototype(&self) -> Option<*mut JSObject> {
        // TODO: Add proper GC to support the below.
        self.slots.prototype
    }

    pub(crate) fn set_prototype(&mut self, prototype: Option<*mut JSObject>) {
        // TODO: Add proper GC to support the below.
        self.slots.prototype = prototype;
    }

    /// Every ordinary object has a Boolean-valued [[Extensible]] internal slot.
    pub(crate) fn ordinary_extensible(&self) -> bool {
        self.slots.extensible.unwrap_or(true)
    }

    pub(crate) fn set_extensible(&mut self, extensible: bool) {
        self.slots.extensible = Some(extensible);
    }

    pub(crate) fn get_property(&self, index: PropertyIndex) -> Option<&JSObjectPropDescriptor> {
        self.values.get(index.0)
    }

    pub(crate) fn has_property(&self, key: &JSObjectPropKey) -> bool {
        self.keys.iter().find(|k| *k == key).is_some()
    }

    pub(crate) fn set_property(
        &mut self,
        key: JSObjectPropKey,
        value: JSObjectPropDescriptor,
    ) -> PropertyIndex {
        self.keys.push(key);
        self.values.push(value);

        PropertyIndex(self.keys.len() - 1)
    }

    pub(crate) fn delete_property(&mut self, index: PropertyIndex) -> bool {
        self.keys.remove(index.0);
        self.values.remove(index.0);

        true
    }

    pub(crate) fn find_property_index(&self, key: &JSObjectPropKey) -> Option<PropertyIndex> {
        self.keys.iter().position(|k| k == key).map(PropertyIndex)
    }
}
