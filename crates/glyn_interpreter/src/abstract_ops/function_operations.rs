use crate::runtime::agent::JSAgent;
use crate::runtime::realm::RealmAddr;
use crate::value::{
    object::{
        internal_slots::{BehaviourFn, JSObjectSlotName},
        property::{JSObjectPropDescriptor, JSObjectPropKey},
        JSObjAddr,
    },
    string::JSString,
    JSValue,
};

use crate::abstract_ops::object_operations::{define_property_or_throw, make_basic_object};

/// 10.2.9 SetFunctionName ( F, name [ , prefix ] )
/// https://262.ecma-international.org/16.0/#sec-setfunctionname
pub(crate) fn set_function_name(
    agent: &mut JSAgent,
    func: JSObjAddr,
    name: JSObjectPropKey,
    opt_prefix: Option<String>,
) {
    // 1. Assert: F is an extensible object that does not have a "name" own property.
    debug_assert!(agent.object(func).extensible() && !agent.object(func).has_property(&name));

    let mut name_str = match name {
        // 2. If name is a Symbol, then
        JSObjectPropKey::Symbol(symbol_name) => {
            // a. Let description be name's [[Description]] value.
            let description = symbol_name.description;

            // a. If description is undefined, set name to the empty String.
            if description.is_none() {
                JSString::from("")
            }
            // b. Else, set name to the string-concatenation of "[", description, and "]".
            else {
                JSString::from(format!(
                    "[{}]",
                    description.unwrap_or_else(|| unreachable!())
                ))
            }
        }
        // 3. Else if name is a Private Name, then
        JSObjectPropKey::PrivateName(private_name) => {
            // a. Set name to name.[[Description]].
            todo!()
        }
        JSObjectPropKey::String(name_str) => name_str,
    };

    // 4. If F has an [[InitialName]] internal slot, then

    if agent.object(func).slots.initial_name().is_some() {
        // a. Set F.[[InitialName]] to name.
        agent
            .object_mut(func)
            .slots
            .set_initial_name(name_str.clone());
    }

    // 5. If prefix is present, then
    if let Some(prefix) = opt_prefix {
        // a. Set name to the string-concatenation of prefix, the code unit 0x0020 (SPACE), and name.
        let new_name = format!("{} {:?}", prefix, &name_str);

        name_str = JSString::from(new_name);
        // b. If F has an [[InitialName]] internal slot, then
        if agent.object(func).slots.initial_name().is_some() {
            // i. Optionally, set F.[[InitialName]] to name.
            agent
                .object_mut(func)
                .slots
                .set_initial_name(name_str.clone());
        }
    }

    // 6. Perform ! DefinePropertyOrThrow(F, "name", PropertyDescriptor { [[Value]]: name, [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }).
    let _ = define_property_or_throw(
        agent,
        func,
        &JSObjectPropKey::String("name".into()),
        JSObjectPropDescriptor {
            value: Some(name_str.into()),
            writable: Some(false),
            enumerable: Some(false),
            configurable: Some(true),
            ..JSObjectPropDescriptor::default()
        },
    );

    // 7. Return unused.
}

/// 10.2.10 SetFunctionLength ( F, length )
/// https://262.ecma-international.org/16.0/#sec-setfunctionlength
pub(crate) fn set_function_length(agent: &mut JSAgent, func: JSObjAddr, length: usize) {
    let length_prop_key = JSObjectPropKey::String("length".into());

    // Assert: F is an extensible object that does not have a "length" own property.
    debug_assert!(
        agent.object(func).extensible() && !agent.object(func).has_property(&length_prop_key)
    );

    // 2. Perform ! DefinePropertyOrThrow(F, "length", PropertyDescriptor { [[Value]]: 𝔽(length), [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }).
    let _ = define_property_or_throw(
        agent,
        func,
        &length_prop_key,
        JSObjectPropDescriptor {
            value: Some(JSValue::from(length as f64)),
            writable: Some(false),
            enumerable: Some(false),
            configurable: Some(true),
            ..JSObjectPropDescriptor::default()
        },
    );

    // 3. Return unused.
}

/// 10.3.4 CreateBuiltinFunction ( behaviour, length, name, additionalInternalSlotsList [ , realm [ , prototype [ , prefix ] ] ] )
/// https://262.ecma-international.org/16.0/#sec-createbuiltinfunction
pub(crate) fn create_builtin_function(
    agent: &mut JSAgent,
    behaviour: BehaviourFn,
    length: usize,
    name: JSObjectPropKey,
    additional_internal_slots: Vec<JSObjectSlotName>,
    opt_realm_addr: Option<RealmAddr>,
    prototype: Option<JSObjAddr>,
    prefix: Option<String>,
) -> JSObjAddr {
    // 1. If realm is not present, set realm to the current Realm Record.
    let realm = opt_realm_addr.unwrap_or_else(|| agent.current_realm());

    // 2. If prototype is not present, set prototype to realm.[[Intrinsics]].[[%Function.prototype%]].
    let prototype = prototype.or(agent.realm(realm).intrinsics.function_prototype);

    // 3. Let internalSlotsList be a List containing the names of all the internal slots that 10.3 requires for the built-in function object that is about to be created.
    let mut internal_slots_list = vec![
        JSObjectSlotName::Prototype,
        JSObjectSlotName::Extensible,
        JSObjectSlotName::Realm,
        JSObjectSlotName::InitialName,
    ];

    // 4. Append to internalSlotsList the elements of additionalInternalSlotsList.
    internal_slots_list.extend(additional_internal_slots);

    // 5. Let func be a new built-in function object that, when called, performs the action described by behaviour using the provided arguments as the values of the corresponding parameters specified by behaviour. The new function object has internal slots whose names are the elements of internalSlotsList, and an [[InitialName]] internal slot.
    let func = make_basic_object(agent, internal_slots_list, None);

    agent.object_mut(func).slots.set_behaviour_fn(behaviour);

    // 6. Set func.[[Prototype]] to prototype.
    agent.object_mut(func).slots.set_prototype(prototype);

    // 7. Set func.[[Extensible]] to true.
    // NOTE: This is the default.

    // 8. Set func.[[Realm]] to realm.
    agent.object_mut(func).slots.set_realm(realm);

    // 9. Set func.[[InitialName]] to null.
    // NOTE: This is the default.

    // 10. Perform SetFunctionLength(func, length).
    set_function_length(agent, func, length);

    // 11. If prefix is not present, then
    // a. Perform SetFunctionName(func, name).
    // a. Perform SetFunctionName(func, name, prefix).
    set_function_name(agent, func, name, prefix);

    // 13. Return func.
    func
}
