mod internal_slots;
mod operations;
mod ordinary;
mod property;

use crate::{
    runtime::CompletionRecord, value::object::internal_slots::ObjectInternalSlots, JSAgent, JSValue,
};

pub use operations::make_basic_object;
pub use property::{JSObjectPropDescriptor, JSObjectPropKey};
use safe_gc::{Collector, Gc, Trace};

/// Essential Internal Methods
/// https://262.ecma-international.org/15.0/index.html#table-essential-internal-methods
#[derive(Debug, PartialEq)]
pub struct JSObjectInternalMethods {
    /// [[GetPrototypeOf]]
    pub get_prototype_of: fn(agent: &JSAgent, object: &JSObject) -> Option<Gc<JSObject>>,
    /// [[SetPrototypeOf]]
    pub set_prototype_of:
        fn(agent: &mut JSAgent, object: &mut JSObject, prototype: Option<Gc<JSObject>>) -> bool,
    /// [[IsExtensible]]
    pub is_extensible: fn(object: &JSObject) -> bool,
    /// [[PreventExtensions]]
    pub prevent_extensions: fn(object: &mut JSObject) -> bool,
    /// [[GetOwnProperty]]
    pub get_own_property:
        fn(object: &JSObject, key: &JSObjectPropKey) -> Option<JSObjectPropDescriptor>,
    /// [[DefineOwnProperty]]
    pub define_own_property: fn(
        agent: &mut JSAgent,
        object: &mut JSObject,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> bool,
    /// [[HasProperty]]
    pub has_property: fn(agent: &JSAgent, object: &JSObject, key: &JSObjectPropKey) -> bool,
    /// [[Get]]
    pub get: fn(
        agent: &JSAgent,
        object: &JSObject,
        key: &JSObjectPropKey,
        receiver: Option<&JSValue>,
    ) -> CompletionRecord,
    /// [[Set]]
    pub set: fn(
        agent: &mut JSAgent,
        object: &mut JSObject,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: Option<&JSValue>,
    ) -> bool,
    /// [[Delete]]
    pub delete: fn(agent: &JSAgent, object: &mut JSObject, key: &JSObjectPropKey) -> bool,
    /// [[OwnPropertyKeys]]
    pub own_property_keys: fn(agent: &JSAgent, object: &JSObject) -> Vec<JSObjectPropKey>,
}

struct PropertyIndex(usize);

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Clone, Debug, PartialEq)]
pub struct JSObject {
    pub methods: &'static JSObjectInternalMethods,
    slots: ObjectInternalSlots,
    keys: Vec<JSObjectPropKey>,
    values: Vec<JSObjectPropDescriptor>,
}

impl Trace for JSObject {
    fn trace(&self, collector: &mut Collector) {
        if let Some(prototype) = self.slots.prototype {
            collector.edge(prototype);
        }
    }
}

impl JSObject {
    /// All ordinary objects have an internal slot called [[Prototype]].
    fn ordinary_prototype(&self) -> Option<Gc<JSObject>> {
        self.slots.prototype
    }

    fn set_prototype(&mut self, prototype: Option<Gc<JSObject>>) {
        self.slots.prototype = prototype;
    }

    /// Every ordinary object has a Boolean-valued [[Extensible]] internal slot.
    fn ordinary_extensible(&self) -> bool {
        self.slots.extensible.unwrap_or(true)
    }

    fn set_extensible(&mut self, extensible: bool) {
        self.slots.extensible = Some(extensible);
    }

    fn keys(&self) -> &[JSObjectPropKey] {
        &self.keys
    }

    fn get_property(&self, index: PropertyIndex) -> Option<&JSObjectPropDescriptor> {
        self.values.get(index.0)
    }

    fn has_property(&self, key: &JSObjectPropKey) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    fn set_property(
        &mut self,
        key: &JSObjectPropKey,
        value: JSObjectPropDescriptor,
    ) -> PropertyIndex {
        self.keys.push(key.clone());
        self.values.push(value);

        PropertyIndex(self.keys.len() - 1)
    }

    fn delete_property(&mut self, index: PropertyIndex) -> bool {
        self.keys.remove(index.0);
        self.values.remove(index.0);

        true
    }

    fn find_property_index(&self, key: &JSObjectPropKey) -> Option<PropertyIndex> {
        self.keys.iter().position(|k| k == key).map(PropertyIndex)
    }
}
