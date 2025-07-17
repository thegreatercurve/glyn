use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{
            function_environment::ThisBindingStatus, Environment, EnvironmentAddr, EnvironmentKind,
        },
        reference::{Reference, ReferenceBase, ReferenceName},
    },
    value::{object::JSObjAddr, string::JSString},
    JSAgent,
};

/// 9.1.2.1 GetIdentifierReference ( env, name, strict )
/// https://262.ecma-international.org/16.0/#sec-getidentifierreference
pub(crate) fn get_identifier_reference(
    agent: &JSAgent,
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
    let exists = (agent.allocator.env(env).methods.has_binding)(agent, env, name)?;

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
    let outer = agent.allocator.env(env).outer;

    // b. Return ? GetIdentifierReference(outer, name, strict).
    get_identifier_reference(agent, outer, name, strict)
}

/// 9.1.2.2 NewDeclarativeEnvironment ( E )
/// https://262.ecma-international.org/16.0/#sec-newdeclarativeenvironment
pub(crate) fn new_declarative_environment(
    agent: &mut JSAgent,
    outer_env: Option<EnvironmentAddr>,
) -> EnvironmentAddr {
    // 1. Let env be a new Declarative Environment Record containing no bindings.
    let mut env = Environment::new(EnvironmentKind::Declarative);

    // 2. Set env.[[OuterEnv]] to E.
    env.outer = outer_env;

    // 3. Return env.
    agent.allocator.alloc(env)
}

/// 9.1.2.3 NewObjectEnvironment ( O, W, E )
/// https://262.ecma-international.org/16.0/#sec-newobjectenvironment
pub(crate) fn new_object_environment(
    agent: &mut JSAgent,
    binding_object: JSObjAddr,
    is_with_environment: bool,
    outer_env: Option<EnvironmentAddr>,
) -> EnvironmentAddr {
    // 1. Let env be a new Object Environment Record.
    let mut env = Environment::new(EnvironmentKind::Object);

    let obj_env = env.obj_env_mut();

    // 2. Set env.[[BindingObject]] to O.
    obj_env.binding_object = Some(binding_object);

    // 3. Set env.[[IsWithEnvironment]] to W.
    obj_env.is_with_environment = is_with_environment;

    // 4. Set env.[[OuterEnv]] to E.
    env.outer = outer_env;

    // 5. Return env.
    agent.allocator.alloc(env)
}

/// 9.1.2.4 NewFunctionEnvironment ( F, newTarget )
/// https://262.ecma-international.org/16.0/#sec-newfunctionenvironment
pub(crate) fn new_function_environment(
    agent: &mut JSAgent,
    function_object_addr: JSObjAddr,
    new_target: Option<JSObjAddr>,
) -> EnvironmentAddr {
    // 1. Let env be a new Function Environment Record containing no bindings.
    let mut env = Environment::new(EnvironmentKind::Function);

    // 2. Set env.[[FunctionObject]] to F.
    env.func_env_mut().function_object = Some(function_object_addr);

    // 3. If F.[[ThisMode]] is lexical, set env.[[ThisBindingStatus]] to lexical.
    // TODO: Implement this using the function object's [[ThisMode]]
    env.func_env_mut().this_binding_status = ThisBindingStatus::Uninitialized;

    // 4. Else, set env.[[ThisBindingStatus]] to uninitialized.
    env.func_env_mut().this_binding_status = ThisBindingStatus::Uninitialized;

    // 5. Set env.[[NewTarget]] to newTarget.
    env.func_env_mut().new_target = new_target;

    // 6. Set env.[[OuterEnv]] to F.[[Environment]].
    env.outer = agent
        .allocator
        .obj(&function_object_addr)
        .slots
        .environment();

    // 7. Return env.
    agent.allocator.alloc(env)
}

/// 9.1.2.5 NewGlobalEnvironment ( G, thisValue )
/// https://262.ecma-international.org/16.0/#sec-newglobalenvironment
pub(crate) fn new_global_environment(
    agent: &mut JSAgent,
    global_object: JSObjAddr,
    this_value: JSObjAddr,
) -> EnvironmentAddr {
    // Note: Object and Declarative Environment Records are created in Environment::new.
    // 2. Let dclRec be NewDeclarativeEnvironment(null).
    // 3. Let env be a new Global Environment Record.
    let mut env = Environment::new(EnvironmentKind::Global);

    // 1. Let objRec be NewObjectEnvironment(G, false, null).
    // Note: Overwrite default binding object with provided global object.
    env.obj_env_mut().binding_object = Some(global_object);

    let global_env = env.global_env_mut();

    // 4. Set env.[[ObjectRecord]] to objRec.
    // 5. Set env.[[GlobalThisValue]] to thisValue.
    global_env.global_this_value = Some(this_value);

    // 6. Set env.[[DeclarativeRecord]] to dclRec.
    // 7. Set env.[[OuterEnv]] to null.
    // 8. Return env.
    agent.allocator.alloc(env)
}
