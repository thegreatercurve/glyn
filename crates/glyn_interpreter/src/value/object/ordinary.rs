use safe_gc::Gc;

use crate::{
    runtime::{normal_completion, CompletionRecord},
    value::{
        comparison::same_value,
        object::{
            operations::create_data_property,
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            JSObjectInternalMethods,
        },
        string::JSString,
        JSObject,
    },
    JSAgent, JSValue,
};

/// 10.1 Ordinary Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots
pub(crate) static ORDINARY_OBJECT_INTERNAL_METHODS: JSObjectInternalMethods =
    JSObjectInternalMethods {
        get_prototype_of,
        set_prototype_of,
        is_extensible,
        prevent_extensions,
        get_own_property,
        define_own_property,
        has_property,
        get,
        set,
        delete,
        own_property_keys,
        call: None,
        construct: None,
    };

/// 10.1.1 [[GetPrototypeOf]] ( )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-getprototypeof
fn get_prototype_of(_agent: &JSAgent, object: &JSObject) -> Option<Gc<JSObject>> {
    // 1. Return OrdinaryGetPrototypeOf(O).
    ordinary_get_prototype_of(object)
}

/// 10.1.1.1 OrdinaryGetPrototypeOf ( O )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarygetprototypeof
fn ordinary_get_prototype_of(object: &JSObject) -> Option<Gc<JSObject>> {
    // 1. Return O.[[Prototype]].
    object.ordinary_prototype()
}

/// 10.1.2 [[SetPrototypeOf]] ( V )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-setprototypeof-v
fn set_prototype_of(
    agent: &mut JSAgent,
    object: &mut JSObject,
    prototype: Option<Gc<JSObject>>,
) -> bool {
    // 1. Return OrdinarySetPrototypeOf(O, V).
    ordinary_set_prototype_of(agent, object, prototype)
}

/// 10.1.2.1 OrdinarySetPrototypeOf ( O, V )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-setprototypeof-v
fn ordinary_set_prototype_of(
    agent: &mut JSAgent,
    object: &mut JSObject,
    prototype: Option<Gc<JSObject>>,
) -> bool {
    // 1. Let current be O.[[Prototype]].
    let current = object.ordinary_prototype();

    // 2. If SameValue(V, current) is true, return true.
    if prototype == current {
        return true;
    }

    // 3. Let extensible be O.[[Extensible]].
    let extensible = object.ordinary_extensible();

    // 4. If extensible is false, return false.
    if !extensible {
        return false;
    }

    // 5. Let p be V.
    let mut opt_p = prototype;

    // 6. Let done be false.
    // 7. Repeat, while done is false,
    while let Some(parent_ptr) = opt_p {
        // a. If p is null, then
        // i. Set done to true.
        // b. Else if SameValue(p, O) is true, then
        let parent = agent.deref_object_ptr(parent_ptr);

        if *object == parent {
            // i. Return false.
            return false;
        }
        // c. Else,
        else {
            // i. If p.[[GetPrototypeOf]] is not the ordinary object internal method defined in 10.1.1, set done to true.
            if parent.methods.get_prototype_of as usize != ordinary_get_prototype_of as usize {
                // i. Set done to true.
                break;
            }

            // ii. Else, set p to p.[[Prototype]].
            opt_p = parent.ordinary_prototype();
        }
    }

    // 8. Set O.[[Prototype]] to V.
    object.set_prototype(opt_p);

    // 9. Return true.
    true
}

/// 10.1.3 [[IsExtensible]] ( )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-isextensible
fn is_extensible(object: &JSObject) -> bool {
    // 1. Return O.[[IsExtensible]]().
    ordinary_is_extensible(object)
}

/// 10.1.3.1 OrdinaryIsExtensible ( O )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinaryisextensible
fn ordinary_is_extensible(object: &JSObject) -> bool {
    // 1. Return O.[[Extensible]].
    object.ordinary_extensible()
}

/// 10.1.4 [[PreventExtensions]] ( )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-preventextensions
fn prevent_extensions(object: &mut JSObject) -> bool {
    // 1. Return OrdinaryPreventExtensions(O).
    ordinary_prevent_extensions(object)
}

/// 10.1.4.1 OrdinaryPreventExtensions ( O )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarypreventextensions
fn ordinary_prevent_extensions(object: &mut JSObject) -> bool {
    // 1. Set O.[[Extensible]] to false.
    object.set_extensible(false);

    // 2. Return true.
    true
}

/// 10.1.5 [[GetOwnProperty]] ( P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-getownproperty-p
fn get_own_property(object: &JSObject, key: &JSObjectPropKey) -> Option<JSObjectPropDescriptor> {
    // 1. Return OrdinaryGetOwnProperty(O, P).
    ordinary_get_own_property(object, key)
}

/// 10.1.5.1 OrdinaryGetOwnProperty ( O, P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarygetownproperty
fn ordinary_get_own_property(
    object: &JSObject,
    key: &JSObjectPropKey,
) -> Option<JSObjectPropDescriptor> {
    // 1. If O does not have an own property with key P, return undefined.
    // 3. Let X be O's own property whose key is P.
    let x = object
        .get_property(object.find_property_index(key)?)?
        .to_owned();

    // 2. Let D be a newly created Property Descriptor with no fields.
    let mut d = JSObjectPropDescriptor::default();

    // 4. If X is a data property, then

    if x.is_data_descriptor() {
        // a. Set D.[[Value]] to the value of X's [[Value]] attribute.
        d.value = x.value.clone();

        // b. Set D.[[Writable]] to the value of X's [[Writable]] attribute.
        d.writable = x.writable;
    } else {
        // a. Assert: X is an accessor property.
        debug_assert!(x.is_accessor_descriptor());

        // b. Set D.[[Get]] to the value of X's [[Get]] attribute.
        d.get = x.get;

        // c. Set D.[[Set]] to the value of X's [[Set]] attribute.
        d.set = x.set;
    }

    // 6. Set D.[[Enumerable]] to the value of X's [[Enumerable]] attribute.
    d.enumerable = x.enumerable;

    // 7. Set D.[[Configurable]] to the value of X's [[Configurable]] attribute.
    d.configurable = x.configurable;

    // 8. Return D.
    Some(d)
}

/// 10.1.6 [[DefineOwnProperty]] ( P, Desc )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarydefineownproperty
fn define_own_property(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    descriptor: JSObjectPropDescriptor,
) -> bool {
    // 1. Return OrdinaryDefineOwnProperty(O, P, Desc).
    ordinary_define_own_property(agent, object, key, descriptor)
}

/// 10.1.6.1 OrdinaryDefineOwnProperty ( O, P, Desc )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarydefineownproperty
fn ordinary_define_own_property(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    descriptor: JSObjectPropDescriptor,
) -> bool {
    // 1. 1. Let current be ? O.[[GetOwnProperty]](P).
    let current = (object.methods.get_own_property)(object, key);

    // 2. 2. Let extensible be ? IsExtensible(O).
    let extensible = is_extensible(object);

    // 3. 3. Return ValidateAndApplyPropertyDescriptor(O, P, extensible, Desc, current).
    validate_and_apply_property_descriptor(
        agent,
        Some(object),
        key,
        extensible,
        descriptor,
        current,
    )
}

/// 10.1.6.2 IsCompatiblePropertyDescriptor ( Extensible, Desc, Current )
/// https://262.ecma-international.org/15.0/index.html#sec-iscompatiblepropertydescriptor
fn is_compatible_property_descriptor(
    agent: &mut JSAgent,
    extensible: bool,
    descriptor: JSObjectPropDescriptor,
    current: Option<JSObjectPropDescriptor>,
) -> bool {
    // 1. Return ValidateAndApplyPropertyDescriptor(undefined, "", Extensible, Desc, Current).
    validate_and_apply_property_descriptor(
        agent,
        None,
        &JSObjectPropKey::String(JSString::from("")),
        extensible,
        descriptor,
        current,
    )
}

/// 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
/// https://262.ecma-international.org/15.0/index.html#sec-validateandapplypropertydescriptor
fn validate_and_apply_property_descriptor(
    agent: &mut JSAgent,
    object: Option<&mut JSObject>,
    key: &JSObjectPropKey,
    extensible: bool,
    descriptor: JSObjectPropDescriptor,
    current: Option<JSObjectPropDescriptor>,
) -> bool {
    // 1. Assert: IsPropertyKey(P) is true.
    // 2. If current is undefined, then
    let Some(current) = current else {
        // a. If extensible is false, return false.
        if !extensible {
            return false;
        }

        // b. If O is undefined, return true.
        let Some(object) = object else {
            return true;
        };

        // c. If IsAccessorDescriptor(Desc) is true, then
        if descriptor.is_accessor_descriptor() {
            // i. Create an own accessor property named P of object O whose [[Get]], [[Set]], [[Enumerable]], and [[Configurable]] attributes are set to the value of the corresponding field in Desc if Desc has that field, or to the attribute's default value otherwise.
            object.set_property(
                key,
                JSObjectPropDescriptor {
                    get: descriptor.get,
                    set: descriptor.set,
                    enumerable: descriptor.enumerable,
                    configurable: descriptor.configurable,
                    ..JSObjectPropDescriptor::default()
                },
            );
        }
        // d. Else,
        else {
            // i. Create an own data property named P of object O whose [[Value]], [[Writable]], [[Enumerable]], and [[Configurable]] attributes are set to the value of the corresponding field in Desc if Desc has that field, or to the attribute's default value otherwise.
            object.set_property(
                key,
                JSObjectPropDescriptor {
                    value: descriptor.value,
                    writable: descriptor.writable,
                    enumerable: descriptor.enumerable,
                    configurable: descriptor.configurable,
                    ..JSObjectPropDescriptor::default()
                },
            );
        }

        // e. Return true.
        return true;
    };

    // 3. Assert: current is a fully populated Property Descriptor.
    debug_assert!(current.is_fully_populated());

    // 4. If Desc does not have any fields, return true.
    if !descriptor.is_fully_populated() {
        return true;
    }

    // 5. If current.[[Configurable]] is false, then
    if current.configurable == Some(false) {
        // a. If Desc has a [[Configurable]] field and Desc.[[Configurable]] is true, return false.
        if descriptor.configurable.is_some() && descriptor.configurable == Some(true) {
            return false;
        }

        // b. If Desc has an [[Enumerable]] field and Desc.[[Enumerable]] is not current.[[Enumerable]], return false.
        if descriptor.enumerable.is_some() && descriptor.enumerable != current.enumerable {
            return false;
        }

        // c. If IsGenericDescriptor(Desc) is false and IsAccessorDescriptor(Desc) is not IsAccessorDescriptor(current), return false.
        if !descriptor.is_generic_descriptor()
            && descriptor.is_accessor_descriptor() != current.is_accessor_descriptor()
        {
            return false;
        }

        // d. If IsAccessorDescriptor(current) is true, then
        if current.is_accessor_descriptor() {
            // i. If Desc has a [[Get]] field and SameValue(Desc.[[Get]], current.[[Get]]) is false, return false.
            if descriptor.get.is_some()
                && !same_value(
                    agent,
                    descriptor.get.as_ref().unwrap_or_else(|| unreachable!()),
                    current.get.as_ref().unwrap_or_else(|| unreachable!()),
                )
            {
                return false;
            }

            // ii. If Desc has a [[Set]] field and SameValue(Desc.[[Set]], current.[[Set]]) is false, return false.
            if descriptor.set.is_some()
                && !same_value(
                    agent,
                    descriptor.set.as_ref().unwrap_or_else(|| unreachable!()),
                    current.set.as_ref().unwrap_or_else(|| unreachable!()),
                )
            {
                return false;
            }
        }
        // e. Else if current.[[Writable]] is false, then
        else if current.writable == Some(false) {
            // i. If Desc has a [[Writable]] field and Desc.[[Writable]] is true, return false.
            if descriptor.writable.is_some() && descriptor.writable == Some(true) {
                return false;
            }

            // ii. If Desc has a [[Value]] field and SameValue(Desc.[[Value]], current.[[Value]]) is false, return false.
            if descriptor.value.is_some()
                && !same_value(
                    agent,
                    descriptor.value.as_ref().unwrap_or_else(|| unreachable!()),
                    current.value.as_ref().unwrap_or_else(|| unreachable!()),
                )
            {
                return false;
            }
        }
    }

    // 6. If O is not undefined, then
    if let Some(object) = object {
        // a. If IsDataDescriptor(current) is true and IsAccessorDescriptor(Desc) is true, then
        if current.is_data_descriptor() && descriptor.is_accessor_descriptor() {
            // i. If Desc has a [[Configurable]] field, let configurable be Desc.[[Configurable]]; else let configurable be current.[[Configurable]].
            let configurable = if descriptor.configurable.is_some() {
                descriptor.configurable.unwrap_or_else(|| unreachable!())
            } else {
                current.configurable.unwrap_or_else(|| unreachable!())
            };

            // ii. If Desc has a [[Enumerable]] field, let enumerable be Desc.[[Enumerable]]; else let enumerable be current.[[Enumerable]].
            let enumerable = if descriptor.enumerable.is_some() {
                descriptor.enumerable.unwrap_or_else(|| unreachable!())
            } else {
                current.enumerable.unwrap_or_else(|| unreachable!())
            };

            // iii. Replace the property named P of object O with an accessor property whose [[Configurable]] and [[Enumerable]] attributes are set to configurable and enumerable, respectively, and whose [[Get]] and [[Set]] attributes are set to the value of the corresponding field in Desc if Desc has that field, or to the attribute's default value otherwise.
            object.set_property(
                key,
                JSObjectPropDescriptor {
                    configurable: Some(configurable),
                    enumerable: Some(enumerable),
                    get: descriptor.get,
                    set: descriptor.set,
                    ..JSObjectPropDescriptor::default()
                },
            );
        }
        // b. Else if IsAccessorDescriptor(current) is true and IsDataDescriptor(Desc) is true, then
        else if current.is_accessor_descriptor() && descriptor.is_data_descriptor() {
            // i. If Desc has a [[Configurable]] field, let configurable be Desc.[[Configurable]]; else let configurable be current.[[Configurable]].
            let configurable = if descriptor.configurable.is_some() {
                descriptor.configurable.unwrap_or_else(|| unreachable!())
            } else {
                current.configurable.unwrap_or_else(|| unreachable!())
            };

            // ii. If Desc has a [[Enumerable]] field, let enumerable be Desc.[[Enumerable]]; else let enumerable be current.[[Enumerable]].
            let enumerable = if descriptor.enumerable.is_some() {
                descriptor.enumerable.unwrap_or_else(|| unreachable!())
            } else {
                current.enumerable.unwrap_or_else(|| unreachable!())
            };

            // iii. Replace the property named P of object O with a data property whose [[Configurable]] and [[Enumerable]] attributes are set to configurable and enumerable, respectively, and whose [[Value]] and [[Writable]] attributes are set to the value of the corresponding field in Desc if Desc has that field, or to the attribute's default value otherwise.
            object.set_property(
                key,
                JSObjectPropDescriptor {
                    configurable: Some(configurable),
                    enumerable: Some(enumerable),
                    value: descriptor.value,
                    writable: descriptor.writable,
                    ..JSObjectPropDescriptor::default()
                },
            );
        }
        // c. Else,
        // i. For each field of Desc, set the corresponding attribute of the property named P of object O to the value of the field.
        else {
            object.set_property(key, descriptor);
        }
    }

    // 7. Return true.
    true
}

/// 10.1.7 [[HasProperty]] ( P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-hasproperty-p
fn has_property(agent: &JSAgent, object: &JSObject, key: &JSObjectPropKey) -> bool {
    // 1. Return OrdinaryHasProperty(O, P).
    ordinary_has_property(agent, object, key)
}

/// 10.1.7.1 OrdinaryHasProperty ( O, P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinaryhasproperty
fn ordinary_has_property(agent: &JSAgent, object: &JSObject, key: &JSObjectPropKey) -> bool {
    // 1. Let hasOwn be ? O.[[GetOwnProperty]](P).
    let has_own = (object.methods.get_own_property)(object, key);

    // 2. If hasOwn is not undefined, return true.
    if has_own.is_some() {
        return true;
    }

    // 3. Let parent be ? O.[[GetPrototypeOf]]().
    let opt_parent_ptr = (object.methods.get_prototype_of)(agent, object);

    // 4. If parent is not null, then
    if let Some(parent_ptr) = opt_parent_ptr {
        // a. Return ? parent.[[HasProperty]](P).
        let parent = agent.deref_object_ptr(parent_ptr);

        return (parent.methods.has_property)(agent, &parent, key);
    }

    // 5. Return false.
    false
}

/// 10.1.8 [[Get]] ( P, Receiver )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-get-p-receiver
fn get(
    agent: &JSAgent,
    object: &JSObject,
    key: &JSObjectPropKey,
    receiver: Option<&JSValue>,
) -> CompletionRecord {
    // 1. Return OrdinaryGet(O, P, Receiver).
    ordinary_get(agent, object, key, receiver)
}

/// 10.1.8.1 OrdinaryGet ( O, P, Receiver )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinaryget
fn ordinary_get(
    agent: &JSAgent,
    object: &JSObject,
    key: &JSObjectPropKey,
    receiver: Option<&JSValue>,
) -> CompletionRecord {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = (object.methods.get_own_property)(object, key);

    // 2. If desc is undefined, then
    let Some(desc) = desc else {
        // a. Let parent be ? O.[[GetPrototypeOf]]().
        let opt_parent_ptr = (object.methods.get_prototype_of)(agent, object);

        // b. If parent is null, return undefined.
        let Some(parent_ptr) = opt_parent_ptr else {
            return normal_completion(JSValue::Undefined);
        };

        // c. Return ? parent.[[Get]](P, Receiver).
        let parent = agent.deref_object_ptr(parent_ptr);

        return (parent.methods.get)(agent, &parent, key, receiver);
    };

    // 3. If IsDataDescriptor(desc) is true, return desc.[[Value]].
    if desc.is_data_descriptor() {
        return normal_completion(desc.value.unwrap_or_else(|| unreachable!()));
    }

    // 4. Assert: IsAccessorDescriptor(desc) is true.
    debug_assert!(desc.is_accessor_descriptor());

    // 5. Let getter be desc.[[Get]].
    let getter = desc.get;

    // 6. If getter is undefined, return undefined.
    if getter.is_none() {
        return normal_completion(JSValue::Undefined);
    }

    // 7. Return ? Call(getter, Receiver).
    todo!()
}

/// 10.1.9 [[Set]] ( P, V, Receiver )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-set-p-v-receiver
fn set(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
    receiver: Option<&JSValue>,
) -> bool {
    // 1. Return OrdinarySet(O, P, V, Receiver).
    ordinary_set(agent, object, key, value, receiver)
}

/// 10.1.9.1 OrdinarySet ( O, P, V, Receiver )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinaryset
fn ordinary_set(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
    receiver: Option<&JSValue>,
) -> bool {
    // 1. Let ownDesc be ? O.[[GetOwnProperty]](P).
    let own_desc = (object.methods.get_own_property)(object, &key);

    // 2. Return ? OrdinarySetWithOwnDescriptor(O, P, V, Receiver, ownDesc).
    ordinary_set_with_own_descriptor(agent, object, key, value, receiver, own_desc)
}

/// 10.1.9.2 OrdinarySetWithOwnDescriptor ( O, P, V, Receiver, ownDesc )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarysetwithowndescriptor
fn ordinary_set_with_own_descriptor(
    agent: &mut JSAgent,
    object: &mut JSObject,
    key: &JSObjectPropKey,
    value: JSValue,
    receiver: Option<&JSValue>,
    opt_own_desc: Option<JSObjectPropDescriptor>,
) -> bool {
    // 1. If ownDesc is undefined, then
    let own_desc = if let Some(own_desc) = opt_own_desc {
        own_desc
    } else {
        // a. Let parent be ? O.[[GetPrototypeOf]]().
        let parent = (object.methods.get_prototype_of)(agent, object);

        // b. If parent is not null, then
        if let Some(parent_ptr) = parent {
            // i. Return ? parent.[[Set]](P, V, Receiver).
            let mut parent = agent.deref_object_ptr(parent_ptr);

            return (parent.methods.set)(agent, &mut parent, key, value, receiver);
        }

        // c. Else,
        // i. Set ownDesc to the PropertyDescriptor { [[Value]]: undefined, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: true }.
        JSObjectPropDescriptor {
            value: Some(JSValue::Undefined),
            writable: Some(true),
            enumerable: Some(true),
            configurable: Some(true),
            ..JSObjectPropDescriptor::default()
        }
    };

    // 2. If IsDataDescriptor(ownDesc) is true, then
    if own_desc.is_data_descriptor() {
        // a. If ownDesc.[[Writable]] is false, return false.
        if own_desc.writable == Some(true) {
            return false;
        }

        // b. If Receiver is not an Object, return false.
        let Some(receiver) = receiver else {
            return false;
        };

        if !receiver.is_object() {
            return false;
        }

        // c. Let existingDescriptor be ? Receiver.[[GetOwnProperty]](P).
        let mut receiver =
            agent.deref_object_ptr(receiver.as_object().unwrap_or_else(|| unreachable!()));

        let existing_desc = (receiver.methods.get_own_property)(&mut receiver, key);

        // d. If existingDescriptor is not undefined, then
        if let Some(existing_desc) = existing_desc {
            // i. If IsAccessorDescriptor(existingDescriptor) is true, return false.
            if existing_desc.is_accessor_descriptor() {
                return false;
            }

            // ii. If existingDescriptor.[[Writable]] is false, return false.
            if existing_desc.writable == Some(false) {
                return false;
            }

            // iii. Let valueDesc be the PropertyDescriptor { [[Value]]: V }.
            let value_desc = JSObjectPropDescriptor {
                value: Some(value),
                ..JSObjectPropDescriptor::default()
            };

            // iv. Return ? Receiver.[[DefineOwnProperty]](P, valueDesc).
            return (receiver.methods.define_own_property)(agent, &mut receiver, key, value_desc);
        }
        // e. Else,
        else {
            // i. Assert: Receiver does not currently have a property P.
            debug_assert!(!receiver.has_property(key));

            // ii. Return ? CreateDataProperty(Receiver, P, V).
            return create_data_property(agent, &mut receiver, key, value);
        }
    }

    // 3. Assert: IsAccessorDescriptor(ownDesc) is true.
    debug_assert!(own_desc.is_accessor_descriptor());

    // 4. Let setter be ownDesc.[[Set]].
    let setter = own_desc.set;

    // 5. If setter is undefined, return false.
    if setter.is_none() {
        return false;
    }

    // 6. Perform ? Call(setter, Receiver, « V »).
    todo!();

    // 7. Return true.
    true
}

/// 10.1.10 [[Delete]] ( P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-delete-p
fn delete(agent: &JSAgent, object: &mut JSObject, key: &JSObjectPropKey) -> bool {
    // 1. Return OrdinaryDelete(O, P).
    ordinary_delete(agent, object, key)
}

/// 10.1.10.1 OrdinaryDelete ( O, P )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinarydelete
fn ordinary_delete(_agent: &JSAgent, object: &mut JSObject, key: &JSObjectPropKey) -> bool {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = (object.methods.get_own_property)(object, key);

    // 2. If desc is undefined, return true.
    let Some(desc) = desc else {
        return true;
    };

    // 3. If desc.[[Configurable]] is true, then
    if desc.configurable.unwrap_or(false) {
        // a. Remove the own property with name P from O.
        object.delete_property(
            object
                .find_property_index(key)
                .unwrap_or_else(|| unreachable!()),
        );

        // b. Return true.
        return true;
    }

    // 4. Return false.
    false
}

/// 10.1.11 [[OwnPropertyKeys]] ( )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinary-object-internal-methods-and-internal-slots-ownpropertykeys
fn own_property_keys(_agent: &JSAgent, object: &JSObject) -> Vec<JSObjectPropKey> {
    // 1. Return OrdinaryOwnPropertyKeys(O).
    ordinary_own_property_keys(object)
}

/// 10.1.11.1 OrdinaryOwnPropertyKeys ( O )
/// https://262.ecma-international.org/15.0/index.html#sec-ordinaryownpropertykeys
fn ordinary_own_property_keys(object: &JSObject) -> Vec<JSObjectPropKey> {
    // Let keys be a new empty List.
    let mut keys: Vec<JSObjectPropKey> = Vec::new();

    // 2. For each own property key P of O such that P is an array index, in ascending numeric index order, do
    for key in object.keys() {
        if key.is_array_index() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // Ascending numeric index order.
    keys.sort_by_key(|key| key.as_array_index().unwrap_or_else(|| unreachable!()));

    // 3. For each own property key P of O such that P is a String and P is not an array index, in ascending chronological order of property creation, do
    for key in object.keys() {
        if key.is_string() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // 4. For each own property key P of O such that P is a Symbol, in ascending chronological order of property creation, do
    for key in object.keys() {
        if key.is_symbol() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // 5. Return keys.
    keys
}
