use crate::{
    abstract_ops::{environments::new_global_environment, ordinary::ordinary_object_create},
    gc::Gc,
    intrinsics::{function_prototype::FunctionPrototype, object_prototype::JSObjectPrototype},
    runtime::{
        agent::JSAgent,
        completion::CompletionRecord,
        execution_context::ExecutionContext,
        intrinsics::Intrinsics,
        realm::{Realm, RealmAddr},
    },
};

/// 9.3.1 InitializeHostDefinedRealm ( )
/// https://262.ecma-international.org/16.0/#sec-initializehostdefinedrealm
pub(crate) fn initialize_host_defined_realm(agent: &mut JSAgent) -> CompletionRecord {
    // 1. Let realm be a new Realm Record.
    let realm = Realm::default();

    let realm_addr = Gc::new(realm);

    // 2. Perform CreateIntrinsics(realm).
    create_intrinsics(agent, realm_addr.clone());

    // 3. Set realm.[[AgentSignifier]] to AgentSignifier().
    // Note: AgentSignifier is not implemented in this codebase, so we skip this step.

    // 4. Set realm.[[TemplateMap]] to a new empty List.
    // Note: TemplateMap is not implemented in this codebase, so we skip this step.

    // 5. Let newContext be a new execution context.
    let new_context = ExecutionContext {
        // 6. Set the Function of newContext to null.
        function: None,

        // 7. Set the Realm of newContext to realm.
        realm: realm_addr.clone(),

        // 8. Set the ScriptOrModule of newContext to null.
        script_or_module: None,

        lexical_environment: None,
        variable_environment: None,
        private_environment: None,
    };

    // 9. Push newContext onto the execution context stack; newContext is now the running execution context.
    agent.push_execution_context(new_context);

    // 10. If the host requires use of an exotic object to serve as realm's global object, then
    // a. Let global be such an object created in a host-defined manner.
    // Note: We don't require exotic objects, so global remains None.

    // 11. Else,
    // a. Let global be OrdinaryObjectCreate(realm.[[Intrinsics]].[[%Object.prototype%]]).
    let global = ordinary_object_create(
        realm_addr.borrow().intrinsics.object_prototype.clone(),
        None,
    );

    // 12. If the host requires that the this binding in realm's global scope return an object other than the global object, then
    // a. Let thisValue be such an object created in a host-defined manner.
    // Note: We don't require special this binding, so thisValue will be global.

    // 13. Else,
    // a. Let thisValue be global.
    let this_value = global.clone();

    // 14. Set realm.[[GlobalObject]] to global.
    realm_addr.borrow_mut().global_object = Some(global.clone());

    // 15. Set realm.[[GlobalEnv]] to NewGlobalEnvironment(global, thisValue).
    realm_addr.borrow_mut().global_env = Some(new_global_environment(&global, &this_value));

    // 16. Perform ? SetDefaultGlobalBindings(realm).
    set_default_global_bindings(&realm_addr)?;

    // 17. Create any host-defined global object properties on global.
    // TODO: Implement this step.

    // 18. Return unused.
    Ok(())
}

/// 9.3.2 CreateIntrinsics ( realmRec )
/// https://262.ecma-international.org/16.0/#sec-createintrinsics
pub(crate) fn create_intrinsics(agent: &mut JSAgent, realm_addr: RealmAddr) -> Intrinsics {
    // 1. Set realmRec.[[Intrinsics]] to a new Record.
    let mut intrinsics = Intrinsics::default();

    // Iniitalize the base object prototype first so it can be used in other intrinsics.
    intrinsics.object_prototype = Some(JSObjectPrototype::create());

    // 2. Set fields of realmRec.[[Intrinsics]] with the values listed in Table 6. The field names are the names listed in column one of the table. The value of each field is a new object value fully and recursively populated with property values as defined by the specification of each object in clauses 19 through 28. All object property values are newly created object values. All values that are built-in function objects are created by performing CreateBuiltinFunction(steps, length, name, slots, realmRec, prototype) where steps is the definition of that function provided by this specification, name is the initial value of the function's "name" property, length is the initial value of the function's "length" property, slots is a list of the names, if any, of the function's specified internal slots, and prototype is the specified value of the function's [[Prototype]] internal slot. The creation of the intrinsics and their properties must be ordered to avoid any dependencies upon objects that have not yet been created.
    intrinsics.function_prototype = Some(FunctionPrototype::create(agent, realm_addr));

    // 3. Perform AddRestrictedFunctionProperties(realmRec.[[Intrinsics]].[[%Function.prototype%]], realmRec).
    // 4. Return unused.

    intrinsics
}

/// 9.3.3 SetDefaultGlobalBindings ( realm )
/// https://262.ecma-international.org/16.0/#sec-setdefaultglobalbindings
fn set_default_global_bindings(_realm: &RealmAddr) -> CompletionRecord {
    // TODO: Implement
    Ok(())
}
