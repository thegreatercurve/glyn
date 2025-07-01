mod internal_slots;
mod operations;
mod ordinary;
mod property;

use crate::{
    runtime::CompletionRecord, value::object::internal_slots::JSObjectInternalSlots, JSAgent,
    JSValue,
};

pub(crate) use operations::call;
pub use operations::make_basic_object;
pub use property::{JSObjectPropDescriptor, JSObjectPropKey};
use safe_gc::{Collector, Gc, Trace};

pub(crate) type JSObjAddr = Gc<JSObject>;

type InternalMethodsCallFn = Option<
    fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        this_value: &JSValue,
        args: &[JSValue],
    ) -> CompletionRecord,
>;

type InternalMethodsConstructFn =
    Option<fn(agent: &mut JSAgent, args: &[JSValue], obj_addr: JSObjAddr) -> JSObjAddr>;

/// Essential Internal Methods
/// https://262.ecma-international.org/15.0/index.html#table-essential-internal-methods
#[derive(Debug, PartialEq)]
pub struct JSObjectInternalMethods {
    /// [[GetPrototypeOf]]
    pub get_prototype_of: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> Option<JSObjAddr>,

    /// [[SetPrototypeOf]]
    pub set_prototype_of:
        fn(agent: &mut JSAgent, obj_addr: JSObjAddr, prototype: Option<JSObjAddr>) -> bool,

    /// [[IsExtensible]]
    pub is_extensible: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> bool,

    /// [[PreventExtensions]]
    pub prevent_extensions: fn(agent: &mut JSAgent, object: JSObjAddr) -> bool,

    /// [[GetOwnProperty]]
    pub get_own_property: fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
    ) -> Option<JSObjectPropDescriptor>,

    /// [[DefineOwnProperty]]
    pub define_own_property: fn(
        agent: &mut JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord,

    /// [[HasProperty]]
    pub has_property: fn(agent: &JSAgent, obj_addr: JSObjAddr, key: &JSObjectPropKey) -> bool,

    /// [[Get]]
    pub get: fn(
        agent: &JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        receiver: &JSValue,
    ) -> CompletionRecord,

    /// [[Set]]
    pub set: fn(
        agent: &mut JSAgent,
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord,

    /// [[Delete]]
    pub delete: fn(agent: &mut JSAgent, obj_addr: JSObjAddr, key: &JSObjectPropKey) -> bool,

    /// [[OwnPropertyKeys]]
    pub own_property_keys: fn(agent: &JSAgent, obj_addr: JSObjAddr) -> Vec<JSObjectPropKey>,

    /// [[Call]]
    pub call: InternalMethodsCallFn,

    /// [[Construct]]
    pub construct: InternalMethodsConstructFn,
}

struct PropertyIndex(usize);

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/15.0/#sec-object-type
#[derive(Debug)]
pub struct JSObject {
    pub methods: &'static JSObjectInternalMethods,
    slots: JSObjectInternalSlots,
    keys: Vec<JSObjectPropKey>,
    values: Vec<JSObjectPropDescriptor>,
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
    fn prototype(&self) -> Option<JSObjAddr> {
        self.slots.prototype()
    }

    /// Every ordinary object has a Boolean-valued [[Extensible]] internal slot.
    pub(crate) fn extensible(&self) -> bool {
        self.slots.extensible()
    }

    // Utility methods for getting and setting properties.
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
