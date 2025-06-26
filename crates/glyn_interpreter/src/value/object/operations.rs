use crate::value::{
    object::{internal_slots::ObjectInternalSlots, ordinary::ORDINARY_OBJECT_INTERNAL_METHODS},
    JSObject,
};

/// 7.3.1 MakeBasicObject ( internalSlotsList )
/// https://262.ecma-international.org/15.0/index.html#sec-makebasicobject
pub fn make_basic_object(_internal_slots_list: &[&str]) -> JSObject {
    // 1. Let obj be a newly created object with an internal slot for each name in internalSlotsList.
    let mut obj = JSObject {
        // 2. Set obj's essential internal methods to the default ordinary object definitions specified in 10.1.
        methods: &ORDINARY_OBJECT_INTERNAL_METHODS,
        slots: ObjectInternalSlots::default(),
        keys: vec![],
        values: vec![],
    };

    // 3. Assert: If the caller will not be overriding both obj's [[GetPrototypeOf]] and [[SetPrototypeOf]] essential internal methods, then internalSlotsList contains [[Prototype]].
    // TODO: Implement internal slots as a hashmap

    // 4. Assert: If the caller will not be overriding all of obj's [[SetPrototypeOf]], [[IsExtensible]], and [[PreventExtensions]] essential internal methods, then internalSlotsList contains [[Extensible]].
    // TODO: Implement internal slots as a hashmap

    // 5. If internalSlotsList contains [[Extensible]], set obj.[[Extensible]] to true.
    obj.slots.extensible = Some(true);

    // 6. Return obj.
    obj
}
