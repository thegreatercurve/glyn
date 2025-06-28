use crate::{
    runtime::CompletionRecord,
    value::{
        object::{internal_slots::ObjectInternalSlots, ordinary::ORDINARY_OBJECT_INTERNAL_METHODS},
        JSObject,
    },
    JSAgent, JSObjectPropDescriptor, JSObjectPropKey, JSValue,
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

/// 7.3.2 Get ( O, P )
/// https://262.ecma-international.org/15.0/index.html#sec-get-o-p
pub(crate) fn get(
    agent: &JSAgent,
    object: &JSObject,
    key: &JSObjectPropKey,
    receiver: Option<&JSValue>,
) -> CompletionRecord {
    // 1. Return ? O.[[Get]](P, O).
    (object.methods.get)(agent, object, key, receiver)
}

/// 7.3.3 GetV ( V, P )
/// https://262.ecma-international.org/15.0/index.html#sec-getv
pub(crate) fn getv(agent: &JSAgent, value: &JSValue, key: &JSObjectPropKey) -> JSValue {
    // 1. Let O be ? ToObject(V).
    // 2. Return ? O.[[Get]](P, V).
    todo!()
}

/// 7.3.4 Set ( O, P, V, Throw )
/// https://262.ecma-international.org/15.0/index.html#sec-set-o-p-v-throw
pub(crate) fn set(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
    throw: bool,
) -> bool {
    // 1. Let success be ? O.[[Set]](P, V, O).
    let success = (object.methods.set)(agent, object, key, value, None);

    // 2. If success is false and Throw is true, throw a TypeError exception.
    if !success && throw {
        agent.type_error("Failed to set property on object");
    }

    // 3. Return unused.
    success
}

/// 7.3.5 CreateDataProperty ( O, P, V )
/// https://262.ecma-international.org/15.0/index.html#sec-createdataproperty
pub(crate) fn create_data_property(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
) -> bool {
    // 1. Let newDesc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: true }.
    let new_desc = JSObjectPropDescriptor {
        value: Some(value),
        writable: Some(true),
        enumerable: Some(true),
        configurable: Some(true),
        ..JSObjectPropDescriptor::default()
    };

    // 2. Return ? O.[[DefineOwnProperty]](P, newDesc).
    (object.methods.define_own_property)(agent, object, key, new_desc)
}

/// 7.3.6 CreateDataPropertyOrThrow ( O, P, V )
/// https://262.ecma-international.org/15.0/index.html#sec-createdatapropertyorthrow
pub(crate) fn create_data_property_or_throw(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
) {
    // 1. 1. Let success be ? CreateDataProperty(O, P, V).
    let success = create_data_property(agent, object, key, value);

    // 2. If success is false, throw a TypeError exception.
    if !success {
        agent.type_error("Failed to create data property on object");
    }

    // 3. Return unused.
}
