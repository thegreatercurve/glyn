use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclEnvironment, object_environment::ObjEnvironment,
            EnvironmentAddr, EnvironmentMethods,
        },
    },
    value::{object::JSObjAddr, string::JSString},
    JSAgent, JSValue,
};

#[derive(Debug)]
pub(crate) struct Binding {
    value: Option<JSValue>,
    mutable: bool,
    deletable: bool,
    strict: bool,
}

/// 9.1.1.4 Global Environment Records
/// https://262.ecma-international.org/16.0/#sec-global-environment-records
#[derive(Debug, Default)]
pub(crate) struct GlobalEnvironment {
    /// [[GlobalThisValue]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-global-environment-records
    global_this_value: Option<JSObjAddr>,
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
            agent.type_error(&format!("Binding already exists for {name:?}"));
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
            agent.type_error(&format!("Binding already exists for {name:?}"));
        }

        // 3. Return ! DclRec.CreateImmutableBinding(N, S).
        DeclEnvironment::create_immutable_binding(agent, env_addr, name, strict)
    }
}

pub(crate) static GLOBAL_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: GlobalEnvironment::has_binding,
    create_mutable_binding: GlobalEnvironment::create_mutable_binding,
    create_immutable_binding: GlobalEnvironment::create_immutable_binding,
};
