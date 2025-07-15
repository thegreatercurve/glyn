use crate::{
    abstract_ops::object_operations::has_property,
    runtime::{
        agent::type_error,
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclEnvironment, object_environment::ObjEnvironment,
            EnvironmentAddr, EnvironmentMethods,
        },
    },
    value::{
        object::{property::JSObjectPropKey, JSObjAddr},
        string::JSString,
    },
    JSAgent, JSValue,
};

/// 9.1.1.4 Global Environment Records
/// https://262.ecma-international.org/16.0/#sec-global-environment-records
#[derive(Debug, Default)]
pub(crate) struct GlobalEnvironment {
    /// [[GlobalThisValue]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-global-environment-records
    pub(crate) global_this_value: Option<JSObjAddr>,
}

impl GlobalEnvironment {
    /// 9.1.1.4.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hasbinding-n
    pub(crate) fn has_binding(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, return true.
        if DeclEnvironment::has_binding(agent, env_addr, name)? {
            return Ok(true);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.HasBinding(N).
        ObjEnvironment::has_binding(agent, env_addr, name)
    }

    /// 9.1.1.4.2 CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-createmutablebinding-n-d
    pub(crate) fn create_mutable_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        deletable: bool,
    ) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, throw a TypeError exception.
        if DeclEnvironment::has_binding(agent, env_addr, &name)? {
            type_error(&format!("Binding already exists for {name:?}"));
        }

        // 3. Return ! DclRec.CreateMutableBinding(N, D).
        DeclEnvironment::create_mutable_binding(agent, env_addr, name, deletable)
    }

    /// 9.1.1.4.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-createimmutablebinding-n-s
    pub(crate) fn create_immutable_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        strict: bool,
    ) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, throw a TypeError exception.
        if DeclEnvironment::has_binding(agent, env_addr, &name)? {
            type_error(&format!("Binding already exists for {name:?}"));
        }

        // 3. Return ! DclRec.CreateImmutableBinding(N, S).
        DeclEnvironment::create_immutable_binding(agent, env_addr, name, strict)
    }

    /// 9.1.1.4.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-initializebinding-n-v
    pub(crate) fn initialize_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
    ) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if DeclEnvironment::has_binding(agent, env_addr, &name)? {
            // a. Return ! DclRec.InitializeBinding(N, V).
            return DeclEnvironment::initialize_binding(agent, env_addr, name, value);
        }

        // 3. Assert: If the binding exists, it must be in the Object Environment Record.
        debug_assert!(ObjEnvironment::has_binding(agent, env_addr, &name)?);

        // 4. Let ObjRec be envRec.[[ObjectRecord]].
        // 5. Return ? ObjRec.InitializeBinding(N, V).
        ObjEnvironment::initialize_binding(agent, env_addr, name, value)
    }

    /// 9.1.1.4.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-setmutablebinding-n-v-s
    pub(crate) fn set_mutable_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if DeclEnvironment::has_binding(agent, env_addr, &name)? {
            // a. Return ? DclRec.SetMutableBinding(N, V, S).
            return DeclEnvironment::set_mutable_binding(agent, env_addr, name, value, strict);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.SetMutableBinding(N, V, S).
        ObjEnvironment::set_mutable_binding(agent, env_addr, name, value, strict)
    }

    /// 9.1.1.4.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-getbindingvalue-n-s
    pub(crate) fn get_binding_value(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
        strict: bool,
    ) -> CompletionRecord<JSValue> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if DeclEnvironment::has_binding(agent, env_addr, name)? {
            // a. Return ? DclRec.GetBindingValue(N, S).
            return DeclEnvironment::get_binding_value(agent, env_addr, name, strict);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.GetBindingValue(N, S).
        ObjEnvironment::get_binding_value(agent, env_addr, name, strict)
    }

    /// 9.1.1.4.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-deletebinding-n
    pub(crate) fn delete_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if DeclEnvironment::has_binding(agent, env_addr, name)? {
            // a. Return ! DclRec.DeleteBinding(N).
            return DeclEnvironment::delete_binding(agent, env_addr, name);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_env = agent.allocator.get(env_addr).obj_env();

        // 4. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_env.binding_object();

        // 5. Let existingProp be ? HasOwnProperty(globalObject, N).
        let existing_prop = has_property(agent, global_object, &JSObjectPropKey::from(name))?;

        // 6. If existingProp is true, then
        if existing_prop {
            // a. Return ? ObjRec.DeleteBinding(N).
            return ObjEnvironment::delete_binding(agent, env_addr, name);
        }

        // 7. Return true.
        Ok(true)
    }

    /// 9.1.1.4.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hasthisbinding
    pub(crate) fn has_this_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        // 1. Return true.
        true
    }

    /// 9.1.1.4.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hassuperbinding
    pub(crate) fn has_super_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.4.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-withbaseobject
    pub(crate) fn with_base_object(
        _agent: &JSAgent,
        _env_addr: EnvironmentAddr,
    ) -> Option<JSObjAddr> {
        // 1. Return undefined.
        None
    }

    /// 9.1.1.4.12 HasLexicalDeclaration ( envRec, N )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-haslexicaldeclaration-envrec-n
    pub(crate) fn has_lexical_declaration(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. Return ! DclRec.HasBinding(N).
        DeclEnvironment::has_binding(agent, env_addr, name)
    }
}

pub(crate) static GLOBAL_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: GlobalEnvironment::has_binding,
    create_mutable_binding: GlobalEnvironment::create_mutable_binding,
    create_immutable_binding: GlobalEnvironment::create_immutable_binding,
    initialize_binding: GlobalEnvironment::initialize_binding,
    set_mutable_binding: GlobalEnvironment::set_mutable_binding,
    get_binding_value: GlobalEnvironment::get_binding_value,
    delete_binding: GlobalEnvironment::delete_binding,
    has_this_binding: GlobalEnvironment::has_this_binding,
    has_super_binding: GlobalEnvironment::has_super_binding,
    with_base_object: GlobalEnvironment::with_base_object,
};
