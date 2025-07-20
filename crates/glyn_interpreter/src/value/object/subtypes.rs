use std::cell::{Ref, RefMut};

use crate::{
    abstract_ops::{
        immutable_prototype_objects::set_immutable_prototype,
        ordinary::{
            ordinary_define_own_property, ordinary_delete, ordinary_get, ordinary_get_own_property,
            ordinary_get_prototype_of, ordinary_has_property, ordinary_is_extensible,
            ordinary_own_property_keys, ordinary_prevent_extensions, ordinary_set,
            ordinary_set_prototype_of,
        },
    },
    runtime::completion::CompletionRecord,
    value::object::{
        property::{JSObjectPropDescriptor, JSObjectPropKey},
        ObjectAddr, ObjectData, ObjectEssentialInternalMethods, ObjectExtraInternalMethods,
        ObjectMeta,
    },
    JSValue,
};

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/16.0/#ordinary-object
pub(crate) struct OrdinaryObject(pub(crate) ObjectAddr);

impl ObjectMeta for OrdinaryObject {
    fn addr(&self) -> ObjectAddr {
        self.0.clone()
    }

    fn data(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }

    fn data_mut(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }
}

/// 10.1 Ordinary Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots
impl ObjectEssentialInternalMethods for OrdinaryObject {
    /// 10.1.1 [[GetPrototypeOf]] ( )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-getprototypeof
    fn get_prototype_of(&self) -> Option<ObjectAddr> {
        // 1. Return OrdinaryGetPrototypeOf(O).
        ordinary_get_prototype_of(self)
    }

    /// 10.1.2 [[SetPrototypeOf]] ( V )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-setprototypeof-v
    fn set_prototype_of(&self, proto_addr: Option<ObjectAddr>) -> bool {
        // 1. Return OrdinarySetPrototypeOf(O, V).
        ordinary_set_prototype_of(self, proto_addr)
    }

    /// 10.1.3 [[IsExtensible]] ( )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-isextensible
    fn is_extensible(&self) -> bool {
        // 1. Return OrdinaryIsExtensible(O).
        ordinary_is_extensible(self)
    }

    /// 10.1.4 [[PreventExtensions]] ( )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-preventextensions
    fn prevent_extensions(&self) -> bool {
        // 1. Return OrdinaryPreventExtensions(O).
        ordinary_prevent_extensions(self)
    }

    /// 10.1.5 [[GetOwnProperty]] ( P )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-getownproperty-p
    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
        // 1. Return OrdinaryGetOwnProperty(O, P).
        ordinary_get_own_property(self, key)
    }

    /// 10.1.6 [[DefineOwnProperty]] ( P, Desc )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-defineownproperty-p-desc
    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool> {
        // 1. Return OrdinaryDefineOwnProperty(O, P, Desc).
        ordinary_define_own_property(self, key, descriptor)
    }

    /// 10.1.7 [[HasProperty]] ( P )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-hasproperty-p
    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        // 1. Return OrdinaryHasProperty(O, P).
        ordinary_has_property(self, key)
    }

    /// 10.1.8 [[Get]] ( P, Receiver )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-get-p-receiver
    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue> {
        // 1. Return OrdinaryGet(O, P, Receiver).
        ordinary_get(self, key, receiver)
    }

    /// 10.1.9 [[Set]] ( P, V, Receiver )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-set-p-v-receiver
    fn set(
        &self,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool> {
        // 1. Return OrdinarySet(O, P, V, Receiver).
        ordinary_set(self, key, value, receiver)
    }

    /// 10.1.10 [[Delete]] ( P )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-delete-p
    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        // 1. Return OrdinaryDelete(O, P).
        ordinary_delete(self, key)
    }

    /// 10.1.11 [[OwnPropertyKeys]] ( )
    /// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-ownpropertykeys
    fn own_property_keys(&self) -> Vec<JSObjectPropKey> {
        // 1. Return OrdinaryOwnPropertyKeys(O).
        ordinary_own_property_keys(self)
    }
}

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/16.0/#function-object
pub(crate) struct FunctionObject(pub(crate) ObjectAddr);

impl ObjectMeta for FunctionObject {
    fn addr(&self) -> ObjectAddr {
        self.0.clone()
    }

    fn data(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }

    fn data_mut(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }
}

impl ObjectEssentialInternalMethods for FunctionObject {
    fn get_prototype_of(&self) -> Option<ObjectAddr> {
        ordinary_get_prototype_of(self)
    }

    fn set_prototype_of(&self, prototype: Option<ObjectAddr>) -> bool {
        ordinary_set_prototype_of(self, prototype)
    }

    fn is_extensible(&self) -> bool {
        ordinary_is_extensible(self)
    }

    fn prevent_extensions(&self) -> bool {
        ordinary_prevent_extensions(self)
    }

    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
        ordinary_get_own_property(self, key)
    }

    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool> {
        ordinary_define_own_property(self, key, descriptor)
    }

    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        ordinary_has_property(self, key)
    }

    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue> {
        ordinary_get(self, key, receiver)
    }

    fn set(
        &self,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool> {
        ordinary_set(self, key, value, receiver)
    }

    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        ordinary_delete(self, key)
    }

    fn own_property_keys(&self) -> Vec<JSObjectPropKey> {
        ordinary_own_property_keys(self)
    }
}

impl ObjectExtraInternalMethods for FunctionObject {
    fn call(&self, _this_value: &JSValue, _args: &[JSValue]) -> CompletionRecord<JSValue> {
        todo!()
    }

    fn construct(
        &self,
        _args: &[JSValue],
        _new_target: &impl ObjectExtraInternalMethods,
    ) -> CompletionRecord<ObjectAddr> {
        todo!()
    }
}

/// 10.4.7 Immutable Prototype Exotic Objects
/// https://262.ecma-international.org/16.0/#sec-immutable-prototype-exotic-objects
pub(crate) struct ImmutablePrototypeExoticObject(pub(crate) ObjectAddr);

/// 10.4.7 Immutable Prototype Exotic Objects
/// https://262.ecma-international.org/16.0/#sec-immutable-prototype-exotic-objects
impl ObjectMeta for ImmutablePrototypeExoticObject {
    fn addr(&self) -> ObjectAddr {
        self.0.clone()
    }

    fn data(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }

    fn data_mut(&self) -> RefMut<ObjectData> {
        self.0.borrow_mut()
    }
}

impl ObjectEssentialInternalMethods for ImmutablePrototypeExoticObject {
    fn get_prototype_of(&self) -> Option<ObjectAddr> {
        ordinary_get_prototype_of(self)
    }

    /// 10.4.7.1 [[SetPrototypeOf]] ( V )
    /// https://262.ecma-international.org/16.0/#sec-immutable-prototype-exotic-objects-setprototypeof-v
    fn set_prototype_of(&self, prototype: Option<ObjectAddr>) -> bool {
        // 1. Return SetImmutablePrototype(O, V).
        set_immutable_prototype(self, prototype)
    }

    fn is_extensible(&self) -> bool {
        ordinary_is_extensible(self)
    }

    fn prevent_extensions(&self) -> bool {
        ordinary_prevent_extensions(self)
    }

    fn get_own_property(
        &self,
        key: &JSObjectPropKey,
    ) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
        ordinary_get_own_property(self, key)
    }

    fn define_own_property(
        &self,
        key: &JSObjectPropKey,
        descriptor: JSObjectPropDescriptor,
    ) -> CompletionRecord<bool> {
        ordinary_define_own_property(self, key, descriptor)
    }

    fn has_property(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        ordinary_has_property(self, key)
    }

    fn get(&self, key: &JSObjectPropKey, receiver: &JSValue) -> CompletionRecord<JSValue> {
        ordinary_get(self, key, receiver)
    }

    fn set(
        &self,
        key: &JSObjectPropKey,
        value: JSValue,
        receiver: JSValue,
    ) -> CompletionRecord<bool> {
        ordinary_set(self, key, value, receiver)
    }

    fn delete(&self, key: &JSObjectPropKey) -> CompletionRecord<bool> {
        ordinary_delete(self, key)
    }

    fn own_property_keys(&self) -> Vec<JSObjectPropKey> {
        ordinary_own_property_keys(self)
    }
}
