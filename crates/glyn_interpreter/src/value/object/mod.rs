pub(crate) mod internal_slots;
pub(crate) mod property;

use crate::{
    gc::Gc,
    runtime::completion::CompletionRecord,
    value::{
        object::{
            internal_slots::JSObjectInternalSlots,
            property::{JSObjectPropDescriptor, JSObjectPropKey},
        },
        JSValue,
    },
};

pub(crate) type JSObjAddr = Gc<JSObject>;

pub(crate) struct JSObjectInternalMethodsVTable {
    pub(crate) get_prototype_of: fn(obj_addr: JSObjAddr) -> Option<JSObjAddr>,

    pub(crate) set_prototype_of: fn(obj_addr: JSObjAddr, value_addr: Option<JSObjAddr>) -> bool,

    pub(crate) is_extensible: fn(obj_addr: JSObjAddr) -> bool,

    pub(crate) prevent_extensions: fn(obj_addr: JSObjAddr) -> bool,

    pub(crate) get_own_property: fn(
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>>,

    pub(crate) define_own_property: fn(
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool>,

    pub(crate) has_property:
        fn(obj_addr: JSObjAddr, key: &JSObjectPropKey) -> CompletionRecord<bool>,

    pub(crate) get: fn(
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        receiver: &JSValue,
    ) -> CompletionRecord<JSValue>,

    pub(crate) set: fn(
        obj_addr: JSObjAddr,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool>,

    pub(crate) delete: fn(obj_addr: JSObjAddr, key: &JSObjectPropKey) -> CompletionRecord<bool>,

    pub(crate) own_property_keys: fn(obj_addr: JSObjAddr) -> Vec<JSObjectPropKey>,
}

/// Essential Internal Methods
/// https://262.ecma-international.org/16.0/#table-essential-internal-methods
pub(crate) trait JSObjectInternalMethods {
    fn v_table(&self) -> JSObjectInternalMethodsVTable;

    /// [[GetPrototypeOf]]
    fn get_prototype_of(&self) -> Option<JSObjAddr>;

    /// [[SetPrototypeOf]]
    fn set_prototype_of(&self, prototype: Option<JSObjAddr>) -> bool;

    /// [[IsExtensible]]
    fn is_extensible(&self) -> bool;

    /// [[PreventExtensions]]
    fn prevent_extensions(&self) -> bool;

    /// [[GetOwnProperty]]
    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>>;

    /// [[DefineOwnProperty]]
    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool>;

    /// [[HasProperty]]
    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool>;

    /// [[Get]]
    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue>;

    /// [[Set]]
    fn set(
        &self,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool>;

    /// [[Delete]]
    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool>;

    /// [[OwnPropertyKeys]]
    fn own_property_keys(&self) -> Vec<JSObjectPropKey>;
}

pub(crate) struct JSObjectExtraInternalMethodsVTable {
    pub(crate) call: Option<
        fn(
            obj_addr: JSObjAddr,
            this_value: &JSValue,
            args: &[JSValue],
        ) -> CompletionRecord<JSValue>,
    >,

    pub(crate) construct: Option<fn(obj_addr: JSObjAddr, args: &[JSValue]) -> JSObjAddr>,
}

/// Additional Essential Internal Methods of Function Objects
/// https://262.ecma-international.org/16.0/#table-additional-essential-internal-methods-of-function-objects
pub(crate) trait JSObjectExtraInternalMethods {
    fn v_table_extra(&self) -> JSObjectExtraInternalMethodsVTable;

    /// [[Call]]
    fn call(&self, this_value: &JSValue, args: &[JSValue]) -> CompletionRecord<JSValue>;

    /// [[Construct]]
    fn construct(&self, args: &[JSValue], obj_addr: JSObjAddr) -> CompletionRecord<JSObjAddr>;
}

pub(crate) struct PropertyIndex(usize);

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/16.0/#sec-object-type
#[derive(Debug)]
pub(crate) struct JSObject {
    pub(crate) slots: JSObjectInternalSlots,
    pub(crate) keys: Vec<JSObjectPropKey>,
    pub(crate) values: Vec<JSObjectPropDescriptor>,
}

impl JSObject {
    /// All ordinary objects have an internal slot called [[Prototype]].
    pub(crate) fn prototype(&self) -> Option<JSObjAddr> {
        self.slots.prototype()
    }

    /// Every ordinary object has a Boolean-valued [[Extensible]] internal slot.
    pub(crate) fn extensible(&self) -> bool {
        self.slots.extensible()
    }

    // Utility methods for getting and setting properties.
    pub(crate) fn keys(&self) -> &[JSObjectPropKey] {
        &self.keys
    }

    pub(crate) fn get_property(&self, index: PropertyIndex) -> Option<&JSObjectPropDescriptor> {
        self.values.get(index.0)
    }

    pub(crate) fn has_property(&self, key: &JSObjectPropKey) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub(crate) fn set_property(
        &mut self,
        key: &JSObjectPropKey,
        value: JSObjectPropDescriptor,
    ) -> PropertyIndex {
        self.keys.push(key.clone());
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
