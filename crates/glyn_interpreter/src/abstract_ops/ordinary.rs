use crate::{
    abstract_ops::{
        object_operations::{call, create_data_property, make_basic_object},
        testing_comparison::{is_extensible, same_value},
    },
    runtime::completion::CompletionRecord,
    value::object::{
        internal_slots::InternalSlotName,
        property::{JSObjectPropDescriptor, JSObjectPropKey},
        ObjectAddr, ObjectEssentialInternalMethods, ObjectMeta,
    },
    JSValue,
};

/// 10.1.1.1 OrdinaryGetPrototypeOf ( O )
/// https://262.ecma-international.org/16.0/#sec-ordinarygetprototypeof
pub(crate) fn ordinary_get_prototype_of<T: ObjectMeta>(object: &T) -> Option<ObjectAddr> {
    // 1. Return O.[[Prototype]].
    object.data().prototype()
}

/// 10.1.2.1 OrdinarySetPrototypeOf ( O, V )
/// https://262.ecma-international.org/16.0/#sec-ordinary-object-internal-methods-and-internal-slots-setprototypeof-v
pub(crate) fn ordinary_set_prototype_of<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    proto: Option<impl ObjectMeta>,
) -> bool {
    let proto_addr = proto.as_ref().map(|addr| addr.addr());

    // 1. Let current be O.[[Prototype]].
    let current = object.get_prototype_of();

    // 2. If SameValue(V, current) is true, return true.
    if proto_addr == current {
        return true;
    }

    // 3. Let extensible be O.[[Extensible]].
    let extensible = object.is_extensible();

    // 4. If extensible is false, return false.
    if !extensible {
        return false;
    }

    // 5. Let p be V.
    let mut opt_p = proto_addr.clone();

    // 6. Let done be false.
    // 7. Repeat, while done is false,
    while let Some(parent) = opt_p {
        // a. If p is null, then
        // i. Set done to true.
        // b. Else if SameValue(p, O) is true, then
        if parent == object.addr() {
            // i. Return false.
            return false;
        }
        // c. Else,
        else {
            // i. If p.[[GetPrototypeOf]] is not the ordinary object internal method defined in 10.1.1, set done to true.
            if !parent.has_ordinary_get_prototype_of() {
                // i. Set done to true.
                break;
            }

            // ii. Else, set p to p.[[Prototype]].
            opt_p = parent.data().prototype();
        }
    }

    // 8. Set O.[[Prototype]] to V.
    object.data_mut().set_prototype(proto_addr);

    // 9. Return true.
    true
}

/// 10.1.3.1 OrdinaryIsExtensible ( O )
/// https://262.ecma-international.org/16.0/#sec-ordinaryisextensible
pub(crate) fn ordinary_is_extensible<T: ObjectMeta>(object: &T) -> bool {
    // 1. Return O.[[Extensible]].
    object.data().extensible
}

/// 10.1.4.1 OrdinaryPreventExtensions ( O )
/// https://262.ecma-international.org/16.0/#sec-ordinarypreventextensions
pub(crate) fn ordinary_prevent_extensions<T: ObjectMeta>(object: &T) -> bool {
    // 1. Set O.[[Extensible]] to false.
    object.data_mut().extensible = false;

    // 2. Return true.
    true
}

/// 10.1.5.1 OrdinaryGetOwnProperty ( O, P )
/// https://262.ecma-international.org/16.0/#sec-ordinarygetownproperty
pub(crate) fn ordinary_get_own_property<T: ObjectMeta>(
    object: &T,
    key: &JSObjectPropKey,
) -> CompletionRecord<Option<JSObjectPropDescriptor>> {
    let object_data = object.data();

    // 1. If O does not have an own property with key P, return undefined.
    // 3. Let X be O's own property whose key is P.
    let Some(x) = object_data.find_property_index(key) else {
        return Ok(None);
    };
    let Some(x) = object_data.get_property(x) else {
        return Ok(None);
    };

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
        d.get = x.get.clone();

        // c. Set D.[[Set]] to the value of X's [[Set]] attribute.
        d.set = x.set.clone();
    }

    // 6. Set D.[[Enumerable]] to the value of X's [[Enumerable]] attribute.
    d.enumerable = x.enumerable;

    // 7. Set D.[[Configurable]] to the value of X's [[Configurable]] attribute.
    d.configurable = x.configurable;

    // 8. Return D.
    Ok(Some(d))
}

/// 10.1.6.1 OrdinaryDefineOwnProperty ( O, P, Desc )
/// https://262.ecma-international.org/16.0/#sec-ordinarydefineownproperty
pub(crate) fn ordinary_define_own_property<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
    descriptor: JSObjectPropDescriptor,
) -> CompletionRecord<bool> {
    // 1. Let current be ? O.[[GetOwnProperty]](P).
    let current = object.get_own_property(key)?;

    // 2. Let extensible be ? IsExtensible(O).
    let extensible = is_extensible(object);

    // 3. Return ValidateAndApplyPropertyDescriptor(O, P, extensible, Desc, current).
    validate_and_apply_property_descriptor(Some(object), key, extensible, descriptor, current);

    Ok(true)
}

/// 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
/// https://262.ecma-international.org/16.0/#sec-validateandapplypropertydescriptor
pub(crate) fn validate_and_apply_property_descriptor<T: ObjectMeta>(
    opt_generic_obj: Option<&T>,
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
        let Some(object) = opt_generic_obj else {
            return true;
        };

        // c. If IsAccessorDescriptor(Desc) is true, then
        if descriptor.is_accessor_descriptor() {
            // i. Create an own accessor property named P of object O whose [[Get]], [[Set]], [[Enumerable]], and [[Configurable]] attributes are set to the value of the corresponding field in Desc if Desc has that field, or to the attribute's default value otherwise.
            object.data_mut().set_property(
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
            object.data_mut().set_property(
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
                    descriptor.get.as_ref().unwrap_or_else(|| unreachable!()),
                    current.get.as_ref().unwrap_or_else(|| unreachable!()),
                )
            {
                return false;
            }

            // ii. If Desc has a [[Set]] field and SameValue(Desc.[[Set]], current.[[Set]]) is false, return false.
            if descriptor.set.is_some()
                && !same_value(
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
                    descriptor.value.as_ref().unwrap_or_else(|| unreachable!()),
                    current.value.as_ref().unwrap_or_else(|| unreachable!()),
                )
            {
                return false;
            }
        }
    }

    // 6. If O is not undefined, then
    if let Some(object) = opt_generic_obj {
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
            object.data_mut().set_property(
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
            object.data_mut().set_property(
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
            object.data_mut().set_property(key, descriptor);
        }
    }

    // 7. Return true.
    true
}

/// 10.1.7.1 OrdinaryHasProperty ( O, P )
/// https://262.ecma-international.org/16.0/#sec-ordinaryhasproperty
pub(crate) fn ordinary_has_property<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
) -> CompletionRecord<bool> {
    // 1. Let hasOwn be ? O.[[GetOwnProperty]](P).
    let has_own = object.get_own_property(key)?;

    // 2. If hasOwn is not undefined, return true.
    if has_own.is_some() {
        return Ok(true);
    }

    // 3. Let parent be ? O.[[GetPrototypeOf]]().
    let opt_parent = object.get_prototype_of();

    // 4. If parent is not null, then
    if let Some(parent) = opt_parent {
        // a. Return ? parent.[[HasProperty]](P).
        return parent.has_property(key);
    }

    // 5. Return false.
    Ok(false)
}

/// 10.1.8.1 OrdinaryGet ( O, P, Receiver )
/// https://262.ecma-international.org/16.0/#sec-ordinaryget
pub(crate) fn ordinary_get<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
    receiver: &JSValue,
) -> CompletionRecord<JSValue> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = object.get_own_property(key)?;

    // 2. If desc is undefined, then
    let Some(desc) = desc else {
        // a. Let parent be ? O.[[GetPrototypeOf]]().
        let opt_parent_addr = object.get_prototype_of();

        // b. If parent is null, return undefined.
        let Some(parent) = opt_parent_addr else {
            return Ok(JSValue::Undefined);
        };

        // c. Return ? parent.[[Get]](P, Receiver).
        return parent.get(key, receiver);
    };

    // 3. If IsDataDescriptor(desc) is true, return desc.[[Value]].
    if desc.is_data_descriptor() {
        return Ok(desc.value.unwrap_or_else(|| unreachable!()));
    }

    // 4. Assert: IsAccessorDescriptor(desc) is true.
    debug_assert!(desc.is_accessor_descriptor());

    // 5. Let getter be desc.[[Get]].
    let getter = desc.get;

    // 6. If getter is undefined, return undefined.
    if getter.is_none() {
        return Ok(JSValue::Undefined);
    }

    // 7. Return ? Call(getter, Receiver).
    call(getter.unwrap_or_else(|| unreachable!()), receiver, None)
}

/// 10.1.9.1 OrdinarySet ( O, P, V, Receiver )
/// https://262.ecma-international.org/16.0/#sec-ordinaryset
pub(crate) fn ordinary_set<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
    value: JSValue,
    receiver: JSValue,
) -> CompletionRecord<bool> {
    // 1. Let ownDesc be ? O.[[GetOwnProperty]](P).
    let own_desc = object.get_own_property(key)?;

    // 2. Return ? OrdinarySetWithOwnDescriptor(O, P, V, Receiver, ownDesc).
    ordinary_set_with_own_descriptor(object, key, value, receiver, own_desc)
}

/// 10.1.9.2 OrdinarySetWithOwnDescriptor ( O, P, V, Receiver, ownDesc )
/// https://262.ecma-international.org/16.0/#sec-ordinarysetwithowndescriptor
pub(crate) fn ordinary_set_with_own_descriptor<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
    value: JSValue,
    receiver: JSValue,
    opt_own_desc: Option<JSObjectPropDescriptor>,
) -> CompletionRecord<bool> {
    // 1. If ownDesc is undefined, then
    let own_desc = if let Some(own_desc) = opt_own_desc {
        own_desc
    } else {
        // a. Let parent be ? O.[[GetPrototypeOf]]().
        let opt_parent = object.get_prototype_of();

        // b. If parent is not null, then
        if let Some(parent) = opt_parent {
            // i. Return ? parent.[[Set]](P, V, Receiver).
            return parent.set(key, value, receiver);
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
            return Ok(false);
        }

        // b. If Receiver is not an Object, return false.
        if !receiver.is_object() {
            return Ok(false);
        }

        // c. Let existingDescriptor be ? Receiver.[[GetOwnProperty]](P).
        let receiver = ObjectAddr::try_from(&receiver)?;

        let existing_desc = receiver.get_own_property(key)?;

        // d. If existingDescriptor is not undefined, then
        if let Some(existing_desc) = existing_desc {
            // i. If IsAccessorDescriptor(existingDescriptor) is true, return false.
            if existing_desc.is_accessor_descriptor() {
                return Ok(false);
            }

            // ii. If existingDescriptor.[[Writable]] is false, return false.
            if existing_desc.writable == Some(false) {
                return Ok(false);
            }

            // iii. Let valueDesc be the PropertyDescriptor { [[Value]]: V }.
            let value_desc = JSObjectPropDescriptor {
                value: Some(value),
                ..JSObjectPropDescriptor::default()
            };

            // iv. Return ? Receiver.[[DefineOwnProperty]](P, valueDesc).
            return receiver.define_own_property(key, value_desc);
        }
        // e. Else,
        else {
            // i. Assert: Receiver does not currently have a property P.
            debug_assert!(!receiver.has_property(key)?);

            // ii. Return ? CreateDataProperty(Receiver, P, V).
            return create_data_property(&receiver, key, value);
        }
    }

    // 3. Assert: IsAccessorDescriptor(ownDesc) is true.
    debug_assert!(own_desc.is_accessor_descriptor());

    // 4. Let setter be ownDesc.[[Set]].
    let setter = own_desc.set;

    // 5. If setter is undefined, return false.
    if setter.is_none() {
        return Ok(false);
    }

    // 6. Perform ? Call(setter, Receiver, « V »).
    call(
        setter.unwrap_or_else(|| unreachable!()),
        &receiver,
        Some(vec![value]),
    )?;

    // 7. Return true.
    Ok(true)
}

/// 10.1.10.1 OrdinaryDelete ( O, P )
/// https://262.ecma-international.org/16.0/#sec-ordinarydelete
pub(crate) fn ordinary_delete<T: ObjectMeta + ObjectEssentialInternalMethods>(
    object: &T,
    key: &JSObjectPropKey,
) -> CompletionRecord<bool> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let desc = object.get_own_property(key)?;

    // 2. If desc is undefined, return true.
    let Some(desc) = desc else {
        return Ok(true);
    };

    // 3. If desc.[[Configurable]] is true, then
    if desc.configurable.unwrap_or(false) {
        // a. Remove the own property with name P from O.
        let property = object
            .data()
            .find_property_index(key)
            .unwrap_or_else(|| unreachable!());

        object.data_mut().delete_property(property);

        // b. Return true.
        return Ok(true);
    }

    // 4. Return false.
    Ok(false)
}

/// 10.1.11.1 OrdinaryOwnPropertyKeys ( O )
/// https://262.ecma-international.org/16.0/#sec-ordinaryownpropertykeys
pub(crate) fn ordinary_own_property_keys<T: ObjectMeta>(object: &T) -> Vec<JSObjectPropKey> {
    // Let keys be a new empty List.
    let mut keys: Vec<JSObjectPropKey> = Vec::new();

    // 2. For each own property key P of O such that P is an array index, in ascending numeric index order, do
    for key in object.data().keys() {
        if key.is_array_index() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // Ascending numeric index order.
    keys.sort_by_key(|key| key.as_array_index().unwrap_or_else(|| unreachable!()));

    // 3. For each own property key P of O such that P is a String and P is not an array index, in ascending chronological order of property creation, do
    for key in object.data().keys() {
        if key.is_string() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // 4. For each own property key P of O such that P is a Symbol, in ascending chronological order of property creation, do
    for key in object.data().keys() {
        if key.is_symbol() {
            // a. Append P to keys.
            keys.push(key.clone());
        }
    }

    // 5. Return keys.
    keys
}

/// 10.1.12 OrdinaryObjectCreate ( proto [ , additionalInternalSlotsList ] )
/// https://262.ecma-international.org/16.0/#sec-ordinaryobjectcreate
pub(crate) fn ordinary_object_create(
    proto_addr: Option<ObjectAddr>,
    additional_internal_slots: Option<Vec<InternalSlotName>>,
) -> ObjectAddr {
    // 1. Let internalSlotsList be « [[Prototype]], [[Extensible]] ».ObjectAddr
    // NOTE: Added directly to the data of the object.
    // 2. If additionalInternalSlotsList is present, set internalSlotsList to the list-concatenation of internalSlotsList and additionalInternalSlotsList.

    // 3. Let O be MakeBasicObject(internalSlotsList).
    let obj = make_basic_object(additional_internal_slots.unwrap_or_default());

    // 4. Set O.[[Prototype]] to proto.
    obj.data_mut().set_prototype(proto_addr);

    // 5. Return O.
    obj
}
