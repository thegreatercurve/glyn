pub(crate) mod internal_slots;
pub(crate) mod property;
pub(crate) mod subtypes;

use std::cell::RefMut;

use crate::{
    gc::Gc,
    runtime::completion::{throw_completion, CompletionRecord, ThrowCompletion},
    value::{
        object::{
            internal_slots::InternalSlots,
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            subtypes::{FunctionObject, ImmutablePrototypeExoticObject, OrdinaryObject},
        },
        JSValue,
    },
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum ObjectKind {
    #[default]
    Ordinary,
    Function,
    ImmutablePrototype,
}

/// 6.1.7 The Object Type
/// https://262.ecma-international.org/16.0/#sec-object-type
#[derive(Debug)]
pub(crate) struct ObjectData {
    // [[Prototype]]
    prototype: Option<ObjectAddr>,

    // [[Extensible]]
    pub(crate) extensible: bool,

    kind: ObjectKind,
    slots: InternalSlots,
    keys: Vec<JSObjectPropKey>,
    values: Vec<JSObjectPropDescriptor>,
}

impl ObjectData {
    pub(crate) fn new(kind: ObjectKind, slots: InternalSlots) -> Self {
        Self {
            kind,
            slots,
            ..Self::default()
        }
    }

    // [[Prototype]]
    pub(crate) fn prototype(&self) -> Option<ObjectAddr> {
        self.prototype.clone()
    }

    /// [[Prototype]]
    pub(crate) fn set_prototype(&mut self, prototype: Option<ObjectAddr>) {
        self.prototype = prototype;
    }

    pub(crate) fn kind(&self) -> &ObjectKind {
        &self.kind
    }

    pub(crate) fn slots(&self) -> &InternalSlots {
        &self.slots
    }

    pub(crate) fn slots_mut(&mut self) -> &mut InternalSlots {
        &mut self.slots
    }

    pub(crate) fn keys(&self) -> &[JSObjectPropKey] {
        &self.keys
    }

    pub(crate) fn values(&self) -> &[JSObjectPropDescriptor] {
        &self.values
    }

    pub(crate) fn get_property(&self, index: usize) -> Option<&JSObjectPropDescriptor> {
        self.values.get(index)
    }

    pub(crate) fn has_property(&self, key: &JSObjectPropKey) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub(crate) fn set_property(
        &mut self,
        key: &JSObjectPropKey,
        value: JSObjectPropDescriptor,
    ) -> usize {
        self.keys.push(key.clone());
        self.values.push(value);

        self.keys.len() - 1
    }

    pub(crate) fn delete_property(&mut self, index: usize) -> bool {
        self.keys.remove(index);
        self.values.remove(index);

        true
    }

    pub(crate) fn find_property_index(&self, key: &JSObjectPropKey) -> Option<usize> {
        self.keys.iter().position(|k| k == key)
    }
}

impl Default for ObjectData {
    fn default() -> Self {
        Self {
            prototype: None,
            extensible: true,
            kind: ObjectKind::Ordinary,
            slots: InternalSlots::default(),
            keys: vec![],
            values: vec![],
        }
    }
}

pub(crate) type ObjectAddr = Gc<ObjectData>;

impl ObjectAddr {
    pub(crate) fn kind(&self) -> ObjectKind {
        self.borrow().kind.clone()
    }
}

impl ObjectMeta for ObjectAddr {
    fn addr(&self) -> ObjectAddr {
        self.clone()
    }

    fn data(&self) -> RefMut<ObjectData> {
        self.borrow_mut()
    }

    fn data_mut(&self) -> RefMut<ObjectData> {
        self.borrow_mut()
    }
}

impl ObjectEssentialInternalMethods for ObjectAddr {
    fn get_prototype_of(&self) -> Option<ObjectAddr> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).get_prototype_of(),
            ObjectKind::Function => FunctionObject::from(self).get_prototype_of(),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).get_prototype_of()
            }
        }
    }

    fn set_prototype_of(&self, prototype: Option<ObjectAddr>) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).set_prototype_of(prototype),
            ObjectKind::Function => FunctionObject::from(self).set_prototype_of(prototype),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).set_prototype_of(prototype)
            }
        }
    }

    fn is_extensible(&self) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).is_extensible(),
            ObjectKind::Function => FunctionObject::from(self).is_extensible(),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).is_extensible()
            }
        }
    }

    fn prevent_extensions(&self) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).prevent_extensions(),
            ObjectKind::Function => FunctionObject::from(self).prevent_extensions(),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).prevent_extensions()
            }
        }
    }

    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).get_own_property(key),
            ObjectKind::Function => FunctionObject::from(self).get_own_property(key),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).get_own_property(key)
            }
        }
    }

    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).define_own_property(key, descriptor),
            ObjectKind::Function => FunctionObject::from(self).define_own_property(key, descriptor),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).define_own_property(key, descriptor)
            }
        }
    }

    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).has_property(key),
            ObjectKind::Function => FunctionObject::from(self).has_property(key),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).has_property(key)
            }
        }
    }

    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).get(key, receiver),
            ObjectKind::Function => FunctionObject::from(self).get(key, receiver),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).get(key, receiver)
            }
        }
    }

    fn set(
        &self,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).set(key, value, receiver),
            ObjectKind::Function => FunctionObject::from(self).set(key, value, receiver),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).set(key, value, receiver)
            }
        }
    }

    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).delete(key),
            ObjectKind::Function => FunctionObject::from(self).delete(key),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).delete(key)
            }
        }
    }

    fn own_property_keys(&self) -> Vec<JSObjectPropKey> {
        match self.kind() {
            ObjectKind::Ordinary => OrdinaryObject::from(self).own_property_keys(),
            ObjectKind::Function => FunctionObject::from(self).own_property_keys(),
            ObjectKind::ImmutablePrototype => {
                ImmutablePrototypeExoticObject::from(self).own_property_keys()
            }
        }
    }
}

impl TryFrom<JSValue> for ObjectAddr {
    type Error = ThrowCompletion;

    fn try_from(value: JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::Object(obj) => Ok(obj),
            _ => throw_completion("Expected JSValue::Object for conversion to ObjectAddr"),
        }
    }
}

impl TryFrom<&JSValue> for ObjectAddr {
    type Error = ThrowCompletion;

    fn try_from(value: &JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::Object(obj) => Ok(obj.clone()),
            _ => throw_completion("Expected JSValue::Object for conversion to ObjectAddr"),
        }
    }
}

impl From<&ObjectAddr> for OrdinaryObject {
    fn from(value: &ObjectAddr) -> Self {
        OrdinaryObject(value.clone())
    }
}

impl From<&ObjectAddr> for FunctionObject {
    fn from(value: &ObjectAddr) -> Self {
        FunctionObject(value.clone())
    }
}

impl From<&ObjectAddr> for ImmutablePrototypeExoticObject {
    fn from(value: &ObjectAddr) -> Self {
        ImmutablePrototypeExoticObject(value.clone())
    }
}

pub(crate) trait ObjectMeta {
    fn addr(&self) -> ObjectAddr;

    fn data(&self) -> RefMut<ObjectData>;

    fn data_mut(&self) -> RefMut<ObjectData>;

    fn has_ordinary_get_prototype_of(&self) -> bool {
        true
    }

    fn is_callable(&self) -> bool {
        false
    }

    fn is_constructor(&self) -> bool {
        false
    }
}

/// Essential Internal Methods
/// https://262.ecma-international.org/16.0/#table-essential-internal-methods
pub(crate) trait ObjectEssentialInternalMethods {
    /// [[GetPrototypeOf]]
    fn get_prototype_of(&self) -> Option<ObjectAddr>;

    /// [[SetPrototypeOf]]
    fn set_prototype_of(&self, prototype: Option<ObjectAddr>) -> bool;

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

/// Additional Essential Internal Methods of Function Objects
/// https://262.ecma-international.org/16.0/#table-additional-essential-internal-methods-of-function-objects
pub(crate) trait ObjectExtraInternalMethods {
    /// [[Call]]
    fn call(&self, this_value: &JSValue, args: &[JSValue]) -> CompletionRecord<JSValue>;

    /// [[Construct]]
    fn construct(
        &self,
        args: &[JSValue],
        new_target: &impl ObjectExtraInternalMethods,
    ) -> CompletionRecord<ObjectAddr>;
}
