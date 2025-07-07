pub(crate)mod internal_slots;
pub(crate)mod property;

use safe_gc::{Collector, Gc, Trace};

use crate::{
    runtime::{agent::JSAgent, completion::CompletionRecord},
    value::{
        object::{
            internal_slots::JSObjectInternalSlots,
            property::{JSObjectPropDescriptor, JSObjectPropKey},
        },
        JSValue,
    },
};

pub(crate)type JSObjAddr = Gc<JSObject>;

type InternalMethodsCallFn = Option<
    fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        this_value: &JSValue,
        args: &[JSValue],
    ) -> CompletionRecord<JSValue>,
>;

type InternalMethodsConstructFn =
    Option<fn(agent: &mut JSAgent, args: &[JSValue], obj_addr: JSObjAddr) -> JSObjAddr>;

/// Essential Internal Methods
/// https://262.ecma-international.org/15.0/#table-essential-internal-methods
#[derive(Debug, PartialEq)]
pub(crate)struct JSObjectInternalMethods {
    /// [[GetPrototypeOf]]
    pub(crate)get_prototype_of: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> Option<JSObjAddr>,

    /// [[SetPrototypeOf]]
    pub(crate)set_prototype_of:
        fn(agent: &mut JSAgent, obj_addr: JSObjAddr, prototype: Option<JSObjAddr>) -> bool,

    /// [[IsExtensible]]
    pub(crate)is_extensible: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> bool,

    /// [[PreventExtensions]]
    pub(crate)prevent_extensions: fn(agent: &mut JSAgent, object: JSObjAddr) -> bool,

    /// [[GetOwnProperty]]
    pub(crate)get_own_property: fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
    ) -> Option<JSObjectPropDescriptor>,

    /// [[DefineOwnProperty]]
    pub(crate)define_own_property: fn(
        agent: &mut JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool>,

    /// [[HasProperty]]
    pub(crate)has_property: fn(agent: &JSAgent, obj_addr: JSObjAddr, key: &JSObjectPropKey) -> bool,

    /// [[Get]]
    pub(crate)get: fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        receiver: &JSValue,
    ) -> CompletionRecord<JSValue>,

    /// [[Set]]
    pub(crate)set: fn(
        agent: &mut JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool>,

    /// [[Delete]]
    pub(crate)delete: fn(agent: &mut JSAgent, obj_addr: JSObjAddr, key: &JSObjectPropKey) -> bool,

    /// [[OwnPropertyKeys]]
    pub(crate)own_property_keys: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> Vec<JSObjectPropKey>,

    /// [[Call]]
    pub(crate)call: InternalMethodsCallFn,

    /// [[Construct]]
    pub(crate)construct: InternalMethodsConstructFn,
}

pub(crate)struct PropertyIndex(usize);

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Debug)]
pub(crate)struct JSObject {
    pub(crate)methods: &'static JSObjectInternalMethods,
    pub(crate)slots: JSObjectInternalSlots,
    pub(crate)keys: Vec<JSObjectPropKey>,
    pub(crate)values: Vec<JSObjectPropDescriptor>,
}

impl Trace for JSObject {
    fn trace(&self, collector: &mut Collector) {
        if let Some(prototype) = self.prototype() {
            collector.edge(prototype);
        }
    }
}

impl JSObject {
    /// All ordinary objects have an internal slot called [[Prototype]].
    pub(crate)fn prototype(&self) -> Option<JSObjAddr> {
        self.slots.prototype()
    }

    /// Every ordinary object has a Boolean-valued [[Extensible]] internal slot.
    pub(crate)fn extensible(&self) -> bool {
        self.slots.extensible()
    }

    // Utility methods for getting and setting properties.
    pub(crate)fn keys(&self) -> &[JSObjectPropKey] {
        &self.keys
    }

    pub(crate)fn get_property(&self, index: PropertyIndex) -> Option<&JSObjectPropDescriptor> {
        self.values.get(index.0)
    }

    pub(crate)fn has_property(&self, key: &JSObjectPropKey) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub(crate)fn set_property(
        &mut self,
        key: &JSObjectPropKey,
        value: JSObjectPropDescriptor,
    ) -> PropertyIndex {
        self.keys.push(key.clone());
        self.values.push(value);

        PropertyIndex(self.keys.len() - 1)
    }

    pub(crate)fn delete_property(&mut self, index: PropertyIndex) -> bool {
        self.keys.remove(index.0);
        self.values.remove(index.0);

        true
    }

    pub(crate)fn find_property_index(&self, key: &JSObjectPropKey) -> Option<PropertyIndex> {
        self.keys.iter().position(|k| k == key).map(PropertyIndex)
    }
}
