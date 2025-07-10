use std::collections::HashMap;

use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{EnvironmentAddr, EnvironmentMethods},
    },
    value::string::JSString,
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
}

pub(crate) static DECLARATIVE_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: DeclEnvironment::has_binding,
    create_mutable_binding: DeclEnvironment::create_mutable_binding,
    create_immutable_binding: DeclEnvironment::create_immutable_binding,
};
