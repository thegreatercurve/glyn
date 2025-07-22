use crate::{
    abstract_ops::{testing_comparison::is_callable, type_conversion::to_object},
    gc::Gc,
    runtime::{agent::type_error, completion::CompletionRecord},
    value::{
        object::{
            internal_slots::{InternalSlotName, InternalSlots},
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            subtypes::FunctionObject,
            ObjectAddr, ObjectData, ObjectEssentialInternalMethods, ObjectExtraInternalMethods,
            ObjectKind, ObjectMeta,
        },
        JSValue,
    },
};

// 7.3 Operations on Objects
// https://262.ecma-international.org/16.0/#sec-operations-on-objects

/// 7.3.1 MakeBasicObject ( internalSlotsList )
/// https://262.ecma-international.org/16.0/#sec-makebasicobject
pub(crate) fn make_basic_object(internal_slots_list: Vec<InternalSlotName>) -> ObjectAddr {
    // 1. Set internalSlotsList to the list-concatenation of internalSlotsList and « [[PrivateElements]] ».
    // 2. Let obj be a newly created object with an internal slot for each name in internalSlotsList.
    // 3. NOTE: As described in Object Internal Methods and Internal Slots, the initial value of each such internal slot is undefined unless specified otherwise.
    // 4. Set obj's essential internal methods to the default ordinary object definitions specified in 10.1.
    let mut obj = ObjectData::new(
        ObjectKind::Ordinary,
        InternalSlots::from(internal_slots_list),
    );

    // 4. Set obj.[[PrivateElements]] to a new empty List.
    // 5. Set obj's essential internal methods to the default ordinary object definitions specified in 10.1.
    // 6. Assert: If the caller will not be overriding both obj's [[GetPrototypeOf]] and [[SetPrototypeOf]] essential internal methods, then internalSlotsList contains [[Prototype]].
    // 7. Assert: If the caller will not be overriding all of obj's [[SetPrototypeOf]], [[IsExtensible]], and [[PreventExtensions]] essential internal methods, then internalSlotsList contains [[Extensible]].

    // 8. If internalSlotsList contains [[Extensible]], set obj.[[Extensible]] to true.
    obj.extensible = true;

    // 9. Return obj.
    Gc::new(obj)
}

/// 7.3.2 Get ( O, P )
/// https://262.ecma-international.org/16.0/#sec-get-o-p
pub(crate) fn get(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
    receiver: &JSValue,
) -> CompletionRecord<JSValue> {
    // 1. Return ? O.[[Get]](P, O).
    object.get(key, receiver)
}

/// 7.3.3 GetV ( V, P )
/// https://262.ecma-international.org/16.0/#sec-getv
pub(crate) fn getv(value: &JSValue, key: &JSObjectPropKey) -> CompletionRecord<JSValue> {
    // 1. Let O be ? ToObject(V).
    let object = to_object(value);

    // 2. Return ? O.[[Get]](P, V).
    object.get(key, value)
}

/// 7.3.4 Set ( O, P, V, Throw )
/// https://262.ecma-international.org/16.0/#sec-set-o-p-v-throw
pub(crate) fn set(
    object: &(impl ObjectMeta + ObjectEssentialInternalMethods),
    key: &JSObjectPropKey,
    value: JSValue,
    throw: bool,
) -> CompletionRecord<Option<bool>> {
    // 1. Let success be ? O.[[Set]](P, V, O).
    let success = object.set(key, value, JSValue::from(object.addr()))?;

    // 2. If success is false and Throw is true, throw a TypeError exception.
    if !success && throw {
        type_error("Failed to set property on object");
    }

    // 3. Return unused.
    Ok(None)
}

/// 7.3.5 CreateDataProperty ( O, P, V )
/// https://262.ecma-international.org/16.0/#sec-createdataproperty
pub(crate) fn create_data_property(
    object: &impl ObjectEssentialInternalMethods,
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
    object.define_own_property(key, new_desc)
}

/// 7.3.6 CreateDataPropertyOrThrow ( O, P, V )
/// https://262.ecma-international.org/16.0/#sec-createdatapropertyorthrow
pub(crate) fn create_data_property_or_throw(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
    value: JSValue,
) -> CompletionRecord {
    // 1. Let success be ? CreateDataProperty(O, P, V).
    let success = create_data_property(object, key, value)?;

    // 2. If success is false, throw a TypeError exception.
    if !success {
        type_error("Failed to create data property on object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.7 CreateNonEnumerableDataPropertyOrThrow ( O, P, V )
/// https://262.ecma-international.org/16.0/#sec-createnonenumerabledatapropertyorthrow
pub(crate) fn create_non_enumerable_data_property_or_throw(
    object: &(impl ObjectMeta + ObjectEssentialInternalMethods),
    key: &JSObjectPropKey,
    value: JSValue,
) {
    // 1. Assert: O is an ordinary, extensible object with no non-configurable properties.
    debug_assert!(
        object.data().extensible
            && object
                .data()
                .values()
                .iter()
                .all(|v| v.configurable == Some(true))
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
    define_property_or_throw(object, key, new_desc).unwrap();

    // 4. Return unused.
}

/// 7.3.8 DefinePropertyOrThrow ( O, P, desc )
/// https://262.ecma-international.org/16.0/#sec-definepropertyorthrow
pub(crate) fn define_property_or_throw(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
    desc: JSObjectPropDescriptor,
) -> CompletionRecord {
    // 1. Let success be ? O.[[DefineOwnProperty]](P, desc).
    let success = object.define_own_property(key, desc)?;

    // 2. If success is false, throw a TypeError exception.
    if !success {
        type_error("Failed to define property on object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.9 DeletePropertyOrThrow ( O, P )
/// https://262.ecma-international.org/16.0/#sec-deletepropertyorthrow
pub(crate) fn delete_property_or_throw(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
) -> CompletionRecord {
    // 1. Let success be ? O.[[Delete]](P).
    let success = object.delete(key)?;

    // 2. If success is false, throw a TypeError exception.
    if !success {
        type_error("Failed to delete property from object");
    }

    // 3. Return unused.
    Ok(())
}

/// 7.3.10 GetMethod ( V, P )
/// https://262.ecma-international.org/16.0/#sec-getmethod
pub(crate) fn get_method(
    value: &JSValue,
    key: &JSObjectPropKey,
) -> CompletionRecord<Option<JSValue>> {
    // 1. Let func be ? GetV(V, P).
    let func = getv(value, key)?;

    // 2. If func is either undefined or null, return undefined.
    if func.is_undefined() || func.is_null() {
        return Ok(None);
    };

    // 3. If IsCallable(func) is false, throw a TypeError exception.
    if !is_callable(&func) {
        type_error("Method is not callable.");
    }

    // 4. Return func.
    Ok(Some(func))
}

/// 7.3.11 HasProperty ( O, P )
/// https://262.ecma-international.org/16.0/#sec-hasproperty
pub(crate) fn has_property(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
) -> CompletionRecord<bool> {
    // 1. Return ? O.[[HasProperty]](P).
    object.has_property(key)
}

/// 7.3.12 HasOwnProperty ( O, P )
/// https://262.ecma-international.org/16.0/#sec-hasownproperty
pub(crate) fn has_own_property(
    object: &impl ObjectEssentialInternalMethods,
    key: &JSObjectPropKey,
) -> CompletionRecord<bool> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = object.get_own_property(key)?;

    // 2. If desc is undefined, return false.
    // 3. Return true.
    Ok(desc.is_some())
}

/// 7.3.13 Call ( F, V [ , argumentsList ] )
/// https://262.ecma-international.org/16.0/#sec-call
pub(crate) fn call(
    function_value: JSValue,
    this_value: &JSValue,
    arguments_list: Option<Vec<JSValue>>,
) -> CompletionRecord<JSValue> {
    // 1. If argumentsList is not present, set argumentsList to a new empty List.
    let args = arguments_list.unwrap_or_default();

    // 2. If IsCallable(F) is false, throw a TypeError exception.
    if !is_callable(&function_value) {
        type_error("Function cannot be called.");
    }

    // 3. Return ? F.[[Call]](V, argumentsList).
    let function_object = FunctionObject::from(&ObjectAddr::try_from(&function_value)?);

    function_object.call(this_value, &args)
}

/// 7.3.14 Construct ( F [ , argumentsList [ , newTarget ] ] )
/// https://262.ecma-international.org/16.0/#sec-construct
pub(crate) fn construct(
    function_obj: &FunctionObject,
    arguments_list: Option<Vec<JSValue>>,
    new_target: Option<&FunctionObject>,
) -> CompletionRecord<ObjectAddr> {
    // 1. If newTarget is not present, set newTarget to F.
    let new_target = new_target.unwrap_or(function_obj);

    // 2. If argumentsList is not present, set argumentsList to a new empty List.
    let arguments_list = arguments_list.unwrap_or_default();

    // 3. Return ? F.[[Construct]](argumentsList, newTarget).
    function_obj.construct(&arguments_list, new_target)
}

/// Integrity level for SetIntegrityLevel operation
#[derive(Debug, PartialEq)]
pub(crate) enum IntegrityLevel {
    Sealed,
    Frozen,
}

/// 7.3.15 SetIntegrityLevel ( O, level )
/// https://262.ecma-international.org/16.0/#sec-setintegritylevel
pub(crate) fn set_integrity_level(
    object: &impl ObjectEssentialInternalMethods,
    level: IntegrityLevel,
) -> CompletionRecord<bool> {
    // 1. Let status be ? O.[[PreventExtensions]]().
    let status = object.prevent_extensions();

    // 2. If status is false, return false.
    if !status {
        return Ok(false);
    }

    // 3. Let keys be ? O.[[OwnPropertyKeys]]().
    let keys = object.own_property_keys();

    // 4. If level is sealed, then
    if matches!(level, IntegrityLevel::Sealed) {
        // a. For each element k of keys, do
        for key in keys {
            // i. Perform ? DefinePropertyOrThrow(O, k, PropertyDescriptor { [[Configurable]]: false }).
            define_property_or_throw(
                object,
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
            let current_desc = object.get_own_property(&key)?;

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
                    define_property_or_throw(object, &key, desc)?;
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
                    define_property_or_throw(object, &key, desc)?;
                }
            }
        }
    }

    // 6. Return true.
    Ok(true)
}

/// 7.3.16 TestIntegrityLevel ( O, level )
/// https://262.ecma-international.org/16.0/#sec-testintegritylevel
pub(crate) fn test_integrity_level(
    object: &impl ObjectEssentialInternalMethods,
    level: IntegrityLevel,
) -> CompletionRecord<bool> {
    // 1. Let extensible be ? IsExtensible(O).
    let extensible = object.is_extensible();

    // 2. If extensible is true, return false.
    if extensible {
        return Ok(false);
    }

    // 3. NOTE: If the object is extensible, none of its properties are examined.
    // 4. Let keys be ? O.[[OwnPropertyKeys]]().
    let keys = object.own_property_keys();

    // 5. For each element k of keys, do
    for key in keys {
        // a. Let currentDesc be ? O.[[GetOwnProperty]](k).
        let current_desc = object.get_own_property(&key)?;

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
