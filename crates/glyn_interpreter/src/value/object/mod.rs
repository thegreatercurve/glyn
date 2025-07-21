pub(crate) mod internal_slots;
pub(crate) mod property;
pub(crate) mod subtypes;

use std::cell::{Ref, RefMut};

use crate::{
    gc::Gc,
    runtime::completion::CompletionRecord,
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
    pub(crate) fn as_ordinary_object(&self) -> OrdinaryObject {
        OrdinaryObject(self.clone())
    }

    pub(crate) fn as_function_object(&self) -> FunctionObject {
        FunctionObject(self.clone())
    }

    pub(crate) fn as_immutable_prototype_object(&self) -> ImmutablePrototypeExoticObject {
        ImmutablePrototypeExoticObject(self.clone())
    }

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
            ObjectKind::Ordinary => self.as_ordinary_object().get_prototype_of(),
            ObjectKind::Function => self.as_function_object().get_prototype_of(),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().get_prototype_of()
            }
        }
    }

    fn set_prototype_of(&self, prototype: Option<ObjectAddr>) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().set_prototype_of(prototype),
            ObjectKind::Function => self.as_function_object().set_prototype_of(prototype),
            ObjectKind::ImmutablePrototype => self
                .as_immutable_prototype_object()
                .set_prototype_of(prototype),
        }
    }

    fn is_extensible(&self) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().is_extensible(),
            ObjectKind::Function => self.as_function_object().is_extensible(),
            ObjectKind::ImmutablePrototype => self.as_immutable_prototype_object().is_extensible(),
        }
    }

    fn prevent_extensions(&self) -> bool {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().prevent_extensions(),
            ObjectKind::Function => self.as_function_object().prevent_extensions(),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().prevent_extensions()
            }
        }
    }

    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().get_own_property(key),
            ObjectKind::Function => self.as_function_object().get_own_property(key),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().get_own_property(key)
            }
        }
    }

    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => self
                .as_ordinary_object()
                .define_own_property(key, descriptor),
            ObjectKind::Function => self
                .as_function_object()
                .define_own_property(key, descriptor),
            ObjectKind::ImmutablePrototype => self
                .as_immutable_prototype_object()
                .define_own_property(key, descriptor),
        }
    }

    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().has_property(key),
            ObjectKind::Function => self.as_function_object().has_property(key),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().has_property(key)
            }
        }
    }

    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue> {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().get(key, receiver),
            ObjectKind::Function => self.as_function_object().get(key, receiver),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().get(key, receiver)
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
            ObjectKind::Ordinary => self.as_ordinary_object().set(key, value, receiver),
            ObjectKind::Function => self.as_function_object().set(key, value, receiver),
            ObjectKind::ImmutablePrototype => self
                .as_immutable_prototype_object()
                .set(key, value, receiver),
        }
    }

    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().delete(key),
            ObjectKind::Function => self.as_function_object().delete(key),
            ObjectKind::ImmutablePrototype => self.as_immutable_prototype_object().delete(key),
        }
    }

    fn own_property_keys(&self) -> Vec<JSObjectPropKey> {
        match self.kind() {
            ObjectKind::Ordinary => self.as_ordinary_object().own_property_keys(),
            ObjectKind::Function => self.as_function_object().own_property_keys(),
            ObjectKind::ImmutablePrototype => {
                self.as_immutable_prototype_object().own_property_keys()
            }
        }
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
