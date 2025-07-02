use crate::{
    abstract_ops::{immutable_prototype_objects, object_operations::make_basic_object},
    value::object::{
        internal_slots::JSObjectSlotName, ordinary::ORDINARY_OBJECT_INTERNAL_METHODS, JSObjAddr,
        JSObjectInternalMethods,
    },
    JSAgent,
};

pub(crate) static IMMUTABLE_OBJECT_INTERNAL_METHODS: JSObjectInternalMethods =
    JSObjectInternalMethods {
        get_prototype_of: ORDINARY_OBJECT_INTERNAL_METHODS.get_prototype_of,
        set_prototype_of: immutable_prototype_objects::set_immutable_prototype,
        is_extensible: ORDINARY_OBJECT_INTERNAL_METHODS.is_extensible,
        prevent_extensions: ORDINARY_OBJECT_INTERNAL_METHODS.prevent_extensions,
        get_own_property: ORDINARY_OBJECT_INTERNAL_METHODS.get_own_property,
        define_own_property: ORDINARY_OBJECT_INTERNAL_METHODS.define_own_property,
        has_property: ORDINARY_OBJECT_INTERNAL_METHODS.has_property,
        get: ORDINARY_OBJECT_INTERNAL_METHODS.get,
        set: ORDINARY_OBJECT_INTERNAL_METHODS.set,
        delete: ORDINARY_OBJECT_INTERNAL_METHODS.delete,
        own_property_keys: ORDINARY_OBJECT_INTERNAL_METHODS.own_property_keys,
        call: None,
        construct: None,
    };

#[derive(Debug)]
pub(crate) struct JSObjectPrototype;

impl JSObjectPrototype {
    pub(crate) fn create(agent: &mut JSAgent) -> JSObjAddr {
        // has an [[Extensible]] internal slot whose value is true.
        // has the internal methods defined for ordinary objects, except for the [[SetPrototypeOf]] method, which is as defined in 10.4.7.1. (Thus, it is an immutable prototype exotic object.)
        // has an internal slot named [[Prototype]] whose value is null.
        make_basic_object(
            agent,
            vec![JSObjectSlotName::Prototype, JSObjectSlotName::Extensible],
            None,
        )
    }
}
