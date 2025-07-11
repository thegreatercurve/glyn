use std::collections::HashMap;

use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{EnvironmentAddr, EnvironmentMethods},
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

/// 9.1.1.1 Declarative Environment Records
/// https://262.ecma-international.org/16.0/#sec-declarative-environment-records
#[derive(Debug, Default)]
pub(crate) struct DeclEnvironment {
    bindings: HashMap<JSString, Binding>,
}

impl DeclEnvironment {
    fn has_binding_internal(&self, name: &JSString) -> bool {
        self.bindings.contains_key(name)
    }

    fn add_binding_internal(
        &mut self,
        name: JSString,
        mutable: bool,
        deletable: bool,
        strict: bool,
    ) {
        self.bindings.insert(
            name,
            Binding {
                mutable,
                deletable,
                strict,
                value: None,
            },
        );
    }

    fn remove_binding_internal(&mut self, name: &JSString) {
        self.bindings.remove(name);
    }
}

impl DeclEnvironment {
    /// 9.1.1.1.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hasbinding-n
    pub(crate) fn has_binding(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. If envRec has a binding for N, return true.
        // 2. Return false.
        Ok(agent
            .environment(env_addr)
            .decl_env
            .as_ref()
            .unwrap()
            .has_binding_internal(name))
    }

    /// 9.1.1.1.2 CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-createmutablebinding-n-d
    pub(crate) fn create_mutable_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        deletable: bool,
    ) -> CompletionRecord {
        let decl_env = agent.environment_mut(env_addr).decl_env.as_mut().unwrap();

        // 1. Assert: envRec does not already have a binding for N.
        debug_assert!(!decl_env.has_binding_internal(&name));

        // 2. Create a mutable binding in envRec for N and record that it is uninitialized. If D is true, record that the newly created binding may be deleted by a subsequent DeleteBinding call.
        decl_env.add_binding_internal(name, true, deletable, true);

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.1.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-createimmutablebinding-n-s
    pub(crate) fn create_immutable_binding(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        strict: bool,
    ) -> CompletionRecord {
        let decl_env = agent.environment_mut(env_addr).decl_env.as_mut().unwrap();

        // 1. Assert: envRec does not already have a binding for N.
        debug_assert!(!decl_env.has_binding_internal(&name));

        // Create an immutable binding in envRec for N and record that it is uninitialized. If S is true, record that the newly created binding is a strict binding.
        decl_env.add_binding_internal(name, false, false, strict);

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.1.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-initializebinding-n-v
    pub(crate) fn initialize_binding(
        _agent: &mut JSAgent,
        _env_addr: EnvironmentAddr,
        _name: JSString,
        _value: JSValue,
    ) -> CompletionRecord {
        todo!()
    }

    /// 9.1.1.1.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-setmutablebinding-n-v-s
    pub(crate) fn set_mutable_binding(
        _agent: &mut JSAgent,
        _env_addr: EnvironmentAddr,
        _name: JSString,
        _value: JSValue,
        _strict: bool,
    ) -> CompletionRecord {
        todo!()
    }

    /// 9.1.1.1.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-getbindingvalue-n-s
    pub(crate) fn get_binding_value(
        _agent: &JSAgent,
        _env_addr: EnvironmentAddr,
        _name: &JSString,
        _strict: bool,
    ) -> CompletionRecord<JSValue> {
        todo!()
    }

    /// 9.1.1.1.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-deletebinding-n
    pub(crate) fn delete_binding(
        _agent: &mut JSAgent,
        _env_addr: EnvironmentAddr,
        _name: &JSString,
    ) -> CompletionRecord<bool> {
        todo!()
    }

    /// 9.1.1.1.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hasthisbinding
    pub(crate) fn has_this_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        todo!()
    }

    /// 9.1.1.1.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hassuperbinding
    pub(crate) fn has_super_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        todo!()
    }

    /// 9.1.1.1.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-withbaseobject
    pub(crate) fn with_base_object(
        _agent: &JSAgent,
        _env_addr: EnvironmentAddr,
    ) -> Option<JSObjAddr> {
        todo!()
    }
}

pub(crate) static DECLARATIVE_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: DeclEnvironment::has_binding,
    create_mutable_binding: DeclEnvironment::create_mutable_binding,
    create_immutable_binding: DeclEnvironment::create_immutable_binding,
    initialize_binding: DeclEnvironment::initialize_binding,
    set_mutable_binding: DeclEnvironment::set_mutable_binding,
    get_binding_value: DeclEnvironment::get_binding_value,
    delete_binding: DeclEnvironment::delete_binding,
    has_this_binding: DeclEnvironment::has_this_binding,
    has_super_binding: DeclEnvironment::has_super_binding,
    with_base_object: DeclEnvironment::with_base_object,
};
