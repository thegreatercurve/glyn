use crate::{
    gc::Gc,
    runtime::{
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclarativeEnvironment,
            function_environment::{FunctionEnvironment, ThisBindingStatus},
            global_environment::GlobalEnvironment,
            object_environment::ObjectEnvironment,
            Environment, EnvironmentAddr, EnvironmentMethods,
        },
        reference::{Reference, ReferenceBase, ReferenceName},
    },
    value::{
        object::{ObjectAddr, ObjectEssentialInternalMethods, ObjectMeta},
        string::JSString,
    },
};

/// 9.1.2.1 GetIdentifierReference ( env, name, strict )
/// https://262.ecma-international.org/16.0/#sec-getidentifierreference
pub(crate) fn get_identifier_reference(
    env: Option<EnvironmentAddr>,
    name: &JSString,
    strict: bool,
) -> CompletionRecord<Reference> {
    // 1. If env is null, then
    let Some(env) = env else {
        // a. Return the Reference Record { [[Base]]: unresolvable, [[ReferencedName]]: name, [[Strict]]: strict, [[ThisValue]]: empty }.
        return Ok(Reference {
            base: ReferenceBase::Unresolvable,
            referenced_name: ReferenceName::from(name),
            strict,
            this_value: None,
        });
    };

    // 2. Let exists be ? env.HasBinding(name).
    let exists = env.has_binding(name)?;

    // 3. If exists is true, then
    if exists {
        // a. Return the Reference Record { [[Base]]: env, [[ReferencedName]]: name, [[Strict]]: strict, [[ThisValue]]: empty }.
        return Ok(Reference {
            base: ReferenceBase::Environment(env),
            referenced_name: ReferenceName::from(name),
            strict,
            this_value: None,
        });
    }

    // 4. Else,
    // a. Let outer be env.[[OuterEnv]].
    let outer = env.outer();

    // b. Return ? GetIdentifierReference(outer, name, strict).
    get_identifier_reference(outer, name, strict)
}

/// 9.1.2.2 NewDeclarativeEnvironment ( E )
/// https://262.ecma-international.org/16.0/#sec-newdeclarativeenvironment
pub(crate) fn new_declarative_environment(outer_env: Option<EnvironmentAddr>) -> EnvironmentAddr {
    // 1. Let env be a new Declarative Environment Record containing no bindings.
    let mut env = DeclarativeEnvironment::default();

    // 2. Set env.[[OuterEnv]] to E.
    env.outer_env = outer_env;

    // 3. Return env.
    Gc::new(Environment::Declarative(env))
}

/// 9.1.2.3 NewObjectEnvironment ( O, W, E )
/// https://262.ecma-international.org/16.0/#sec-newobjectenvironment
pub(crate) fn new_object_environment(
    binding_object: &(impl ObjectMeta + ObjectEssentialInternalMethods),
    is_with_environment: bool,
    outer_env: Option<EnvironmentAddr>,
) -> EnvironmentAddr {
    // 1. Let env be a new Object Environment Record.
    let env = ObjectEnvironment {
        // 2. Set env.[[BindingObject]] to O.
        binding_object: binding_object.addr(),

        // 3. Set env.[[IsWithEnvironment]] to W.
        is_with_environment,

        // 4. Set env.[[OuterEnv]] to E.
        outer_env,
    };

    // 5. Return env.
    Gc::new(Environment::Object(env))
}

/// 9.1.2.4 NewFunctionEnvironment ( F, newTarget )
/// https://262.ecma-international.org/16.0/#sec-newfunctionenvironment
pub(crate) fn new_function_environment(
    function_obj: &(impl ObjectMeta + ObjectEssentialInternalMethods),
    new_target: Option<ObjectAddr>,
) -> EnvironmentAddr {
    // 1. Let env be a new Function Environment Record containing no bindings.
    let env = FunctionEnvironment {
        // 2. Set env.[[FunctionObject]] to F.
        function_object: Some(function_obj.addr()),

        // 3. If F.[[ThisMode]] is lexical, set env.[[ThisBindingStatus]] to lexical.
        // TODO: Implement this using the function object's [[ThisMode]]
        // 4. Else, set env.[[ThisBindingStatus]] to uninitialized.
        this_binding_status: ThisBindingStatus::Uninitialized,

        // 5. Set env.[[NewTarget]] to newTarget.
        new_target,

        // 6. Set env.[[OuterEnv]] to F.[[Environment]].
        outer_env: function_obj.data().slots().environment(),

        decl_env: DeclarativeEnvironment::default(),

        this_value: None,
    };

    // 7. Return env.
    Gc::new(Environment::Function(env))
}

/// 9.1.2.5 NewGlobalEnvironment ( G, thisValue )
/// https://262.ecma-international.org/16.0/#sec-newglobalenvironment
pub(crate) fn new_global_environment(
    global_object: &ObjectAddr,
    this_value: &ObjectAddr,
) -> EnvironmentAddr {
    // Let objRec be NewObjectEnvironment(G, false, null).
    let obj_rec = ObjectEnvironment {
        binding_object: global_object.addr(),
        is_with_environment: false,
        outer_env: None,
    };

    // 2. Let dclRec be NewDeclarativeEnvironment(null).
    let decl_env = DeclarativeEnvironment::default();

    // 3. Let env be a new Global Environment Record.
    let env = GlobalEnvironment {
        // 4. Set env.[[ObjectRecord]] to objRec.
        object_record: obj_rec,

        // 5. Set env.[[GlobalThisValue]] to thisValue.
        global_this_value: Some(this_value.addr()),

        // 6. Set env.[[DeclarativeRecord]] to dclRec.
        declarative_record: decl_env,

        // 7. Set env.[[OuterEnv]] to null.
        outer_env: None,
    };

    // 8. Return env.
    Gc::new(Environment::Global(env))
}
