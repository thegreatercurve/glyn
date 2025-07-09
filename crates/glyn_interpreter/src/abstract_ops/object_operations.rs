use crate::value::{
    object::{
        internal_slots::{JSObjectInternalSlots, JSObjectSlotName},
        property::{JSObjectPropDescriptor, JSObjectPropKey},
        JSObjAddr, JSObject, JSObjectInternalMethods,
    },
    JSValue,
};

use crate::abstract_ops::{
    object::ORDINARY_OBJECT_INTERNAL_METHODS, testing_comparison::is_callable,
    type_conversion::to_object,
};

use crate::runtime::agent::JSAgent;
use crate::runtime::completion::CompletionRecord;

// 7.3 Operations on Objects
// https://262.ecma-international.org/15.0/#sec-operations-on-objects

/// 7.3.1 MakeBasicObject ( internalSlotsList )
/// https://262.ecma-international.org/15.0/#sec-makebasicobject
pub(crate) fn make_basic_object(
    agent: &mut JSAgent,
    internal_slots_list: Vec<JSObjectSlotName>,
    internal_methods: Option<&'static JSObjectInternalMethods>,
) -> JSObjAddr {
    // 1. Let obj be a newly created object with an internal slot for each name in internalSlotsList.
    let mut obj = JSObject {
        // 2. Set obj's essential internal methods to the default ordinary object definitions specified in 10.1.
        methods: internal_methods.unwrap_or(&ORDINARY_OBJECT_INTERNAL_METHODS),
        slots: JSObjectInternalSlots::from(internal_slots_list),
        keys: vec![],
        values: vec![],
    };

    // 3. Assert: If the caller will not be overriding both obj's [[GetPrototypeOf]] and [[SetPrototypeOf]] essential internal methods, then internalSlotsList contains [[Prototype]].
    // 4. Assert: If the caller will not be overriding all of obj's [[SetPrototypeOf]], [[IsExtensible]], and [[PreventExtensions]] essential internal methods, then internalSlotsList contains [[Extensible]].

    // 5. If internalSlotsList contains [[Extensible]], set obj.[[Extensible]] to true.
    obj.slots.set_extensible(true);

    // 6. Return obj.
    agent.allocate_object(obj)
}

/// 7.3.2 Get ( O, P )
/// https://262.ecma-international.org/15.0/#sec-get-o-p
pub(crate) fn get(
    agent: &JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    receiver: &JSValue,
) -> CompletionRecord<JSValue> {
    // 1. Return ? O.[[Get]](P, O).
    (agent.object(obj_addr).methods.get)(agent, obj_addr, key, receiver)
}

/// 7.3.3 GetV ( V, P )
/// https://262.ecma-international.org/15.0/#sec-getv
pub(crate) fn getv(
    agent: &JSAgent,
    value: &JSValue,
    key: &JSObjectPropKey,
) -> CompletionRecord<JSValue> {
    // 1. Let O be ? ToObject(V).
    let obj_addr = to_object(agent, value);

    // 2. Return ? O.[[Get]](P, V).
    (agent.object(obj_addr).methods.get)(agent, obj_addr, key, value)
}

/// 7.3.4 Set ( O, P, V, Throw )
/// https://262.ecma-international.org/15.0/#sec-set-o-p-v-throw
pub(crate) fn set(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
    throw: bool,
) -> CompletionRecord<Option<bool>> {
    // 1. Let success be ? O.[[Set]](P, V, O).
    let success =
        (agent.object(obj_addr).methods.set)(agent, obj_addr, key, value, obj_addr.into())?;

    // 2. If success is false and Throw is true, throw a TypeError exception.
    if !success && throw {
        agent.type_error("Failed to set property on object");
    }

    // 3. Return unused.
    Ok(None)
}

/// 7.3.5 CreateDataProperty ( O, P, V )
/// https://262.ecma-international.org/15.0/#sec-createdataproperty
pub(crate) fn create_data_property(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
) -> CompletionRecord<bool> {
    // 1. Let newDesc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: true }.
    let new_desc = JSObjectPropDescriptor {
        value: Some(value),
        writable: Some(true),
        enumerable: Some(true),
        configurable: Some(true),
        ..JSObjectPropDescriptor::default()
    };

    // 2. Return ? O.[[DefineOwnProperty]](P, newDesc).
    (agent.object(obj_addr).methods.define_own_property)(agent, obj_addr, key, new_desc)
}

/// 7.3.6 CreateDataPropertyOrThrow ( O, P, V )
/// https://262.ecma-international.org/15.0/#sec-createdatapropertyorthrow
pub(crate) fn create_data_property_or_throw(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
) -> CompletionRecord<()> {
    // 1. Let success be ? CreateDataProperty(O, P, V).
    let success = create_data_property(agent, obj_addr, key, value)?;

    // 2. If success is false, throw a TypeError exception.
    if !success {
        agent.type_error("Failed to create data property on object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.7 CreateNonEnumerableDataPropertyOrThrow ( O, P, V )
/// https://262.ecma-international.org/15.0/#sec-createnonenumerabledatapropertyorthrow
pub(crate) fn create_non_enumerable_data_property_or_throw(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
) {
    let object = agent.object_mut(obj_addr);

    // 1. Assert: O is an ordinary, extensible object with no non-configurable properties.
    debug_assert!(
        object.extensible() && object.values.iter().all(|v| v.configurable == Some(true))
    );

    // 2. Let newDesc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true, [[Enumerable]]: false, [[Configurable]]: true }.
    let new_desc = JSObjectPropDescriptor {
        value: Some(value),
        writable: Some(true),
        enumerable: Some(false),
        configurable: Some(true),
        ..JSObjectPropDescriptor::default()
    };

    // 3. Perform ! DefinePropertyOrThrow(O, P, newDesc).
    let _ = define_property_or_throw(agent, obj_addr, key, new_desc);

    // 4. Return unused.
}

/// 7.3.8 DefinePropertyOrThrow ( O, P, desc )
/// https://262.ecma-international.org/15.0/#sec-definepropertyorthrow
pub(crate) fn define_property_or_throw(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    desc: JSObjectPropDescriptor,
) -> CompletionRecord<()> {
    // 1. Let success be ? O.[[DefineOwnProperty]](P, desc).
    let success = (agent.object(obj_addr).methods.define_own_property)(agent, obj_addr, key, desc)?;

    // 2. If success is false, throw a TypeError exception.
    if !success {
        agent.type_error("Failed to define property on object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.9 DeletePropertyOrThrow ( O, P )
/// https://262.ecma-international.org/15.0/#sec-deletepropertyorthrow
pub(crate) fn delete_property_or_throw(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
) -> CompletionRecord<()> {
    // 1. Let success be ? O.[[Delete]](P).
    let success = (agent.object(obj_addr).methods.delete)(agent, obj_addr, key);

    // 2. If success is false, throw a TypeError exception.
    if !success {
        agent.type_error("Failed to delete property from object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.10 GetMethod ( V, P )
/// https://262.ecma-international.org/15.0/#sec-getmethod
pub(crate) fn get_method(
    agent: &JSAgent,
    value: &JSValue,
    key: &JSObjectPropKey,
) -> CompletionRecord<Option<JSValue>> {
    // 1. Let func be ? GetV(V, P).
    let func = getv(agent, value, key)?;

    // 2. If func is either undefined or null, return undefined.
    if func.is_undefined() || func.is_null() {
        return Ok(None);
    };

    // 3. If IsCallable(func) is false, throw a TypeError exception.
    if !is_callable(agent, &func) {
        agent.type_error("Method is not callable.");
    }

    // 4. Return func.
    Ok(Some(func))
}

/// 7.3.11 HasProperty ( O, P )
/// https://262.ecma-international.org/15.0/#sec-hasproperty
pub(crate) fn has_property(agent: &JSAgent, obj_addr: JSObjAddr, key: &JSObjectPropKey) -> bool {
    // 1. Return ? O.[[HasProperty]](P).
    (agent.object(obj_addr).methods.has_property)(agent, obj_addr, key)
}

/// 7.3.12 HasOwnProperty ( O, P )
/// https://262.ecma-international.org/15.0/#sec-hasownproperty
pub(crate) fn has_own_property(
    agent: &JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
) -> bool {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = (agent.object(obj_addr).methods.get_own_property)(agent, obj_addr, key);

    // 2. If desc is undefined, return false.
    // 3. Return true.
    desc.is_some()
}

/// 7.3.13 Call ( F, V [ , argumentsList ] )
/// https://262.ecma-international.org/15.0/#sec-call
pub(crate) fn call(
    agent: &JSAgent,
    function_value: JSValue,
    this_value: &JSValue,
    arguments_list: Option<Vec<JSValue>>,
) -> CompletionRecord<JSValue> {
    // 1. If argumentsList is not present, set argumentsList to a new empty List.
    let args = arguments_list.unwrap_or_default();

    // 2. If IsCallable(F) is false, throw a TypeError exception.
    if !is_callable(agent, &function_value) {
        agent.type_error("Function cannot be called.");
    }

    // 3. Return ? F.[[Call]](V, argumentsList).
    let function_obj_addr = function_value.as_object().unwrap_or_else(|| unreachable!());

    let call_fn = agent.object(function_obj_addr).methods.call.unwrap();

    call_fn(agent, function_obj_addr, this_value, &args)
}

/// 7.3.14 Construct ( F [ , argumentsList [ , newTarget ] ] )
/// https://262.ecma-international.org/15.0/#sec-construct
pub(crate) fn construct(
    agent: &mut JSAgent,
    constructor: JSObjAddr,
    arguments_list: Option<Vec<JSValue>>,
    new_target: Option<JSObjAddr>,
) -> CompletionRecord<JSObjAddr> {
    // 1. If newTarget is not present, set newTarget to F.
    let new_target_addr = new_target.unwrap_or(constructor);

    // 2. If argumentsList is not present, set argumentsList to a new empty List.
    let arguments_list = arguments_list.unwrap_or_default();

    // 3. Return ? F.[[Construct]](argumentsList, newTarget).
    let construct_fn = agent.object(constructor).methods.construct.unwrap();

    let result = construct_fn(agent, &arguments_list, new_target_addr);

    Ok(result)
}

/// Integrity level for SetIntegrityLevel operation
#[derive(Debug, PartialEq)]
pub(crate) enum IntegrityLevel {
    Sealed,
    Frozen,
}

/// 7.3.15 SetIntegrityLevel ( O, level )
/// https://262.ecma-international.org/15.0/#sec-setintegritylevel
pub(crate) fn set_integrity_level(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    level: IntegrityLevel,
) -> CompletionRecord<bool> {
    // 1. Let status be ? O.[[PreventExtensions]]().
    let status = (agent.object(obj_addr).methods.prevent_extensions)(agent, obj_addr);

    // 2. If status is false, return false.
    if !status {
        return Ok(false);
    }

    // 3. Let keys be ? O.[[OwnPropertyKeys]]().
    let keys = (agent.object(obj_addr).methods.own_property_keys)(agent, obj_addr);

    // 4. If level is sealed, then
    if matches!(level, IntegrityLevel::Sealed) {
        // a. For each element k of keys, do
        for key in keys {
            // i. Perform ? DefinePropertyOrThrow(O, k, PropertyDescriptor { [[Configurable]]: false }).
            let _ = define_property_or_throw(
                agent,
                obj_addr,
                &key,
                JSObjectPropDescriptor {
                    configurable: Some(false),
                    ..JSObjectPropDescriptor::default()
                },
            )?;
        }
    }
    // 5. Else,
    else {
        // a. Assert: level is frozen.
        debug_assert!(matches!(level, IntegrityLevel::Frozen));

        // b. For each element k of keys, do
        for key in keys {
            // i. Let currentDesc be ? O.[[GetOwnProperty]](k).
            let current_desc =
                (agent.object(obj_addr).methods.get_own_property)(agent, obj_addr, &key);

            // ii. If currentDesc is not undefined, then
            if let Some(current_desc) = current_desc {
                // 1. If IsAccessorDescriptor(currentDesc) is true, then
                if current_desc.is_accessor_descriptor() {
                    // a. Let desc be the PropertyDescriptor { [[Configurable]]: false }.
                    let desc = JSObjectPropDescriptor {
                        configurable: Some(false),
                        ..JSObjectPropDescriptor::default()
                    };

                    // 3. Perform ? DefinePropertyOrThrow(O, k, desc).
                    define_property_or_throw(agent, obj_addr, &key, desc)?;
                }
                // 2. Else,
                else {
                    // a. Let desc be the PropertyDescriptor { [[Configurable]]: false, [[Writable]]: false }.
                    let desc = JSObjectPropDescriptor {
                        configurable: Some(false),
                        writable: Some(false),
                        ..JSObjectPropDescriptor::default()
                    };

                    // 3. Perform ? DefinePropertyOrThrow(O, k, desc).
                    define_property_or_throw(agent, obj_addr, &key, desc)?;
                }
            }
        }
    }

    // 6. Return true.
    Ok(true)
}

/// 7.3.16 TestIntegrityLevel ( O, level )
/// https://262.ecma-international.org/15.0/#sec-testintegritylevel
pub(crate) fn test_integrity_level(
    agent: &JSAgent,
    obj_addr: JSObjAddr,
    level: IntegrityLevel,
) -> CompletionRecord<bool> {
    // 1. Let extensible be ? IsExtensible(O).
    let extensible = (agent.object(obj_addr).methods.is_extensible)(agent, obj_addr);

    // 2. If extensible is true, return false.
    if extensible {
        return Ok(false);
    }

    // 3. NOTE: If the object is extensible, none of its properties are examined.
    // 4. Let keys be ? O.[[OwnPropertyKeys]]().
    let keys = (agent.object(obj_addr).methods.own_property_keys)(agent, obj_addr);

    // 5. For each element k of keys, do
    for key in keys {
        // a. Let currentDesc be ? O.[[GetOwnProperty]](k).
        let current_desc = (agent.object(obj_addr).methods.get_own_property)(agent, obj_addr, &key);

        // b. If currentDesc is not undefined, then
        if let Some(current_desc) = current_desc {
            // i. If currentDesc.[[Configurable]] is true, return false.
            if current_desc.configurable == Some(true) {
                return Ok(false);
            }

            // ii. If level is frozen and IsDataDescriptor(currentDesc) is true, then
            if level == IntegrityLevel::Frozen && current_desc.is_data_descriptor() {
                // 1. If currentDesc.[[Writable]] is true, return false.
                if current_desc.writable == Some(true) {
                    return Ok(false);
                }
            }
        }
    }

    // 6. Return true.
    Ok(true)
}
