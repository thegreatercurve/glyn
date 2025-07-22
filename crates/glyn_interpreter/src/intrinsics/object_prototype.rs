use crate::{
    gc::Gc,
    value::object::{internal_slots::InternalSlots, ObjectAddr, ObjectData, ObjectKind},
};

/// 20.1.3 Properties of the Object Prototype Object
/// https://262.ecma-international.org/16.0/#sec-properties-of-the-object-prototype-object
#[derive(Debug)]
pub(crate) struct JSObjectPrototype;

impl JSObjectPrototype {
    pub(crate) fn create() -> ObjectAddr {
        // is %Object.prototype%.
        // has an [[Extensible]] internal slot whose value is true.
        // has the internal methods defined for ordinary objects, except for the [[SetPrototypeOf]] method, which is as defined in 10.4.7.1. (Thus, it is an immutable prototype exotic object.)
        // has a [[Prototype]] internal slot whose value is null.
        Gc::new(ObjectData::new(
            ObjectKind::ImmutablePrototype,
            InternalSlots::default(),
        ))
    }
}
