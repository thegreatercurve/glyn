use crate::{
    abstract_ops::object_operations::make_basic_object,
    runtime::{completion::CompletionRecord, realm::Realm},
    value::object::{internal_slots::JSObjectSlotName, JSObjAddr},
    JSAgent,
};

/// 10.3.4 CreateBuiltinFunction ( behaviour, length, name, additionalInternalSlotsList [ , realm [ , prototype [ , prefix ] ] ] )
/// https://262.ecma-international.org/15.0/#sec-createbuiltinfunction
pub(crate) fn create_builtin_function(
    agent: &mut JSAgent,
    behaviour: JSObjAddr,
    length: usize,
    name: Option<String>,
    additional_internal_slots: Vec<JSObjectSlotName>,
    realm: Option<Box<Realm>>,
    prototype: Option<JSObjAddr>,
    prefix: Option<String>,
) -> CompletionRecord {
    let _ = additional_internal_slots;
    // 1. If realm is not present, set realm to the current Realm Record.
    let realm = realm.unwrap_or_else(|| agent.current_realm());

    // 2. If prototype is not present, set prototype to realm.[[Intrinsics]].[[%Function.prototype%]].
    let prototype = prototype.or(realm.intrinsics.function_prototype);

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

    // 6. Set func.[[Prototype]] to prototype.
    agent.object_mut(func).slots.set_prototype(prototype);

    // 7. Set func.[[Extensible]] to true.
    // NOTE: This is the default.

    // 8. Set func.[[Realm]] to realm.
    agent.object_mut(func).slots.set_realm(realm);

    // 9. Set func.[[InitialName]] to null.
    // NOTE: This is the default.

    // 10. Perform SetFunctionLength(func, length).
    // 11. If prefix is not present, then
    // a. Perform SetFunctionName(func, name).
    // 12. Else,
    // a. Perform SetFunctionName(func, name, prefix).
    // 13. Return func.
    todo!()
}
