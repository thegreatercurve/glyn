use crate::{
    abstract_ops::{
        immutable_prototype_objects::{self, set_immutable_prototype},
        object_operations::make_basic_object,
    },
    runtime::agent::JSAgent,
    value::object::{internal_slots::InternalSlotName, JSObjAddr, JSObjectInternalMethods},
};

/// 20.1.3 Properties of the Object Prototype Object
/// https://262.ecma-international.org/16.0/#sec-properties-of-the-object-prototype-object
#[derive(Debug)]
pub(crate) struct JSObjectPrototype;

impl JSObjectPrototype {
    pub(crate) fn create(agent: &mut JSAgent) -> JSObjAddr {
        // has an [[Extensible]] internal slot whose value is true.
        // has the internal methods defined for ordinary objects, except for the [[SetPrototypeOf]] method, which is as defined in 10.4.7.1. (Thus, it is an immutable prototype exotic object.)

        // has an internal slot named [[Prototype]] whose value is null.
        let mut obj_addr = make_basic_object(
            agent,
            vec![InternalSlotName::Prototype, InternalSlotName::Extensible],
        );

        obj_addr.v_table().set_prototype_of = set_immutable_prototype;

        obj_addr
    }
}
