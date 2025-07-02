use crate::{
    abstract_ops::testing_comparison::is_callable,
    runtime::{
        agent::JSAgent,
        completion::{CompletionRecord, NormalCompletion},
    },
    value::{
        object::{
            internal_slots::{JSObjectInternalSlots, JSObjectSlotName},
            ordinary::ORDINARY_OBJECT_INTERNAL_METHODS,
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            JSObjAddr, JSObject,
        },
        JSValue,
    },
};

// 7.3 Operations on Objects
// https://262.ecma-international.org/15.0/#sec-operations-on-objects

/// 7.3.1 MakeBasicObject ( internalSlotsList )
/// https://262.ecma-international.org/15.0/#sec-makebasicobject
pub fn make_basic_object(
    agent: &mut JSAgent,
    internal_slots_list: Vec<JSObjectSlotName>,
) -> JSObjAddr {
    // 1. Let obj be a newly created object with an internal slot for each name in internalSlotsList.
    let mut obj = JSObject {
        // 2. Set obj's essential internal methods to the default ordinary object definitions specified in 10.1.
        methods: &ORDINARY_OBJECT_INTERNAL_METHODS,
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
) -> CompletionRecord {
    // 1. Return ? O.[[Get]](P, O).
    (agent.object(obj_addr).methods.get)(agent, obj_addr, key, receiver)
}

/// 7.3.3 GetV ( V, P )
/// https://262.ecma-international.org/15.0/#sec-getv
pub(crate) fn getv(agent: &JSAgent, value: &JSValue, key: &JSObjectPropKey) -> CompletionRecord {
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
) -> CompletionRecord {
    // 1. Let success be ? O.[[Set]](P, V, O).
    let success =
        (agent.object(obj_addr).methods.set)(agent, obj_addr, key, value, obj_addr.into());

    // 2. If success is false and Throw is true, throw a TypeError exception.

    if (success.is_err() || success.is_ok_and(|value| value == JSValue::Boolean(false).into()))
        && throw
    {
        agent.type_error("Failed to set property on object");
    }

    // 3. Return unused.
    Ok(NormalCompletion::Unused)
}

/// 7.3.5 CreateDataProperty ( O, P, V )
/// https://262.ecma-international.org/15.0/#sec-createdataproperty
pub(crate) fn create_data_property(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
) -> CompletionRecord {
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
    object_addr: JSObjAddr,
    key: &JSObjectPropKey,
    value: JSValue,
) -> CompletionRecord {
    // 1. 1. Let success be ? CreateDataProperty(O, P, V).
    let success = create_data_property(agent, object_addr, key, value);

    // 2. If success is false, throw a TypeError exception.
    if success.is_err() || success.is_ok_and(|value| value == JSValue::Boolean(false).into()) {
        agent.type_error("Failed to create data property on object");
    }

    // 3. Return unused.
    Ok(NormalCompletion::Unused)
}

/// 7.3.10 GetMethod ( V, P )
/// https://262.ecma-international.org/15.0/#sec-getmethod
pub(crate) fn get_method(
    agent: &JSAgent,
    value: &JSValue,
    key: &JSObjectPropKey,
) -> CompletionRecord {
    // 1. Let func be ? GetV(V, P).
    let func = getv(agent, value, key);

    // 2. If func is either undefined or null, return undefined.
    let Ok(NormalCompletion::Value(ref func_value)) = func else {
        return Ok(NormalCompletion::Value(JSValue::Undefined));
    };

    // 3. If IsCallable(func) is false, throw a TypeError exception.
    if !is_callable(agent, func_value) {
        agent.type_error("Method is not callable.");
    }

    // 4. Return func.
    func
}

/// 7.3.13 Call ( F, V [ , argumentsList ] )
/// https://262.ecma-international.org/15.0/#sec-call
pub(crate) fn call(
    agent: &JSAgent,
    function_value: JSValue,
    this_value: &JSValue,
    arguments_list: Option<Vec<JSValue>>,
) -> CompletionRecord {
    // 1. If argumentsList is not present, set argumentsList to a new empty List.
    let args = arguments_list.unwrap_or_default();

    // 2. If IsCallable(F) is false, throw a TypeError exception.
    if !is_callable(agent, &function_value) {
        agent.type_error("Function cannot be called.");
    }

    // 3. Return ? F.[[Call]](V, argumentsList).
    let function_obj_addr = function_value.as_object().unwrap_or_else(|| unreachable!());

    let call_fn = agent
        .object(function_obj_addr)
        .methods
        .call
        .unwrap_or_else(|| unreachable!());

    call_fn(agent, function_obj_addr, &this_value, &args)
}

/// 7.1.18 ToObject ( argument )
/// https://262.ecma-international.org/15.0/#sec-toobject
pub(crate) fn to_object(agent: &JSAgent, arg: &JSValue) -> JSObjAddr {
    match arg {
        JSValue::Undefined => {
            // Throw a TypeError exception.
            agent.type_error("Cannot convert undefined to object");
        }
        JSValue::Null => {
            // Throw a TypeError exception.
            agent.type_error("Cannot convert null to object");
        }
        // Return a new Boolean object whose [[BooleanData]] internal slot is set to argument.
        JSValue::Boolean(_value) => todo!(),
        // Return a new Number object whose [[NumberData]] internal slot is set to argument.
        JSValue::Number(_value) => todo!(),
        // Return a new String object whose [[StringData]] internal slot is set to argument.
        JSValue::String(_value) => todo!(),
        // Return a new Symbol object whose [[SymbolData]] internal slot is set to argument.
        JSValue::Symbol => todo!(),
        // Return a new BigInt object whose [[BigIntData]] internal slot is set to argument.
        JSValue::BigInt(_value) => todo!(),
        // If argument is an Object, return argument.
        JSValue::Object(addr) => *addr,
    }
}
