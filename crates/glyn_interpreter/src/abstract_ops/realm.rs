use std::rc::Rc;

use glyn_execution_model::{
    agent::JSAgent, completion::CompletionRecord, execution_context::ExecutionContext,
    intrinsics::Intrinsics, realm::Realm, value::object::JSObjAddr,
};

use crate::{
    abstract_ops::{environments::new_global_environment, object::ordinary_object_create},
    intrinsics::{function_prototype::FunctionPrototype, object_prototype::JSObjectPrototype},
};

/// 9.3.1 CreateRealm ( )
/// https://262.ecma-international.org/15.0/#sec-createrealm
pub fn create_realm(agent: &mut JSAgent) -> Realm {
    // 1. Let realmRec be a new Realm Record.
    let realm_rec = Realm {
        // 2. Perform CreateIntrinsics(realmRec).
        intrinsics: create_intrinsics(agent),

        // 4. Set realmRec.[[GlobalObject]] to undefined.
        global_object: None,

        // 5. Set realmRec.[[GlobalEnv]] to undefined.
        global_env: None,
    };

    // 7. Return realmRec.
    realm_rec
}

/// 9.3.2 CreateIntrinsics ( realmRec )
/// https://262.ecma-international.org/15.0/#sec-createintrinsics
pub fn create_intrinsics(agent: &mut JSAgent) -> Intrinsics {
    // 1. Set realmRec.[[Intrinsics]] to a new Record.
    let mut intrinsics = Intrinsics::default();

    // Iniitalize the base object prototype first so it can be used in other intrinsics.
    intrinsics.object_prototype = Some(JSObjectPrototype::create(agent));

    // 2. Set fields of realmRec.[[Intrinsics]] with the values listed in Table 6. The field names are the names listed in column one of the table. The value of each field is a new object value fully and recursively populated with property values as defined by the specification of each object in clauses 19 through 28. All object property values are newly created object values. All values that are built-in function objects are created by performing CreateBuiltinFunction(steps, length, name, slots, realmRec, prototype) where steps is the definition of that function provided by this specification, name is the initial value of the function's "name" property, length is the initial value of the function's "length" property, slots is a list of the names, if any, of the function's specified internal slots, and prototype is the specified value of the function's [[Prototype]] internal slot. The creation of the intrinsics and their properties must be ordered to avoid any dependencies upon objects that have not yet been created.
    intrinsics.function_prototype = Some(FunctionPrototype::create(agent));

    // 3. Perform AddRestrictedFunctionProperties(realmRec.[[Intrinsics]].[[%Function.prototype%]], realmRec).
    // 4. Return unused.

    intrinsics
}

/// 9.3.3 SetRealmGlobalObject ( realmRec, globalObj, thisValue )
/// https://262.ecma-international.org/15.0/#sec-setrealmglobalobject
pub fn set_realm_global_object(
    agent: &mut JSAgent,
    realm_record: &mut Realm,
    opt_global_obj_addr: Option<JSObjAddr>,
    this_value: Option<JSObjAddr>,
) {
    // 1. If globalObj is undefined, then
    let global_obj_addr = opt_global_obj_addr.unwrap_or_else(|| {
        // a. Let intrinsics be realmRec.[[Intrinsics]].
        let intrinsics = &realm_record.intrinsics;

        // b. Set globalObj to OrdinaryObjectCreate(intrinsics.[[%Object.prototype%]]).
        ordinary_object_create(agent, intrinsics.object_prototype, None)
    });

    // 2. Assert: globalObj is an Object.
    // 3. If thisValue is undefined, set thisValue to globalObj.
    let this_value = this_value.unwrap_or(global_obj_addr);

    // 4. Set realmRec.[[GlobalObject]] to globalObj.
    realm_record.global_object = Some(global_obj_addr);

    // 5. Let newGlobalEnv be NewGlobalEnvironment(globalObj, thisValue).
    let new_global_env = new_global_environment(agent, global_obj_addr, this_value);

    // 6. Set realmRec.[[GlobalEnv]] to newGlobalEnv.
    realm_record.global_env = Some(Rc::new(new_global_env));

    // 7. Return unused.
}

/// 9.6 InitializeHostDefinedRealm ( )
/// https://262.ecma-international.org/15.0/#sec-initializehostdefinedrealm
pub(crate) fn initialize_host_defined_realm(
    agent: &mut JSAgent,
    realm_record: &mut Realm,
) -> CompletionRecord {
    // 1. Let realm be CreateRealm().
    let realm = create_realm(agent);

    // 2. Let newContext be a new execution context.
    let new_context = ExecutionContext {
        // 3. Set the Function of newContext to null.
        function: None,

        // 4. Set the Realm of newContext to realm.
        realm: Rc::new(realm),

        // 5. Set the ScriptOrModule of newContext to null.
        script_or_module: None,

        ..ExecutionContext::default()
    };

    // 6. Push newContext onto the execution context stack; newContext is now the running execution context.
    agent.push_execution_context(new_context);

    // 7. If the host requires use of an exotic object to serve as realm's global object, let global be such an object created in a host-defined manner. Otherwise, let global be undefined, indicating that an ordinary object should be created as the global object.
    let global = None;

    // 8. If the host requires that the this binding in realm's global scope return an object other than the global object, let thisValue be such an object created in a host-defined manner. Otherwise, let thisValue be undefined, indicating that realm's global this binding should be the global object.
    let this_value = None;

    // 9. Perform SetRealmGlobalObject(realm, global, thisValue).
    set_realm_global_object(agent, realm_record, global, this_value);

    // 10. Let globalObj be ? SetDefaultGlobalBindings(realm).
    let global_obj = set_default_global_bindings(agent, realm_record)?;

    // 11. Create any host-defined global object properties on globalObj.
    todo!();

    // 12. Return unused.
    Ok(())
}

/// 9.3.4 SetDefaultGlobalBindings ( realm )
/// https://262.ecma-international.org/15.0/#sec-setdefaultglobalbindings
fn set_default_global_bindings(agent: &mut JSAgent, realm: &Realm) -> CompletionRecord<JSObjAddr> {
    todo!()
}
