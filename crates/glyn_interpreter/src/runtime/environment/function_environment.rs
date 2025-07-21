use crate::{
    runtime::{
        agent::reference_error,
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclEnvironment, EnvironmentAddr, EnvironmentMethods,
        },
    },
    value::{
        object::{ObjectAddr, ObjectEssentialInternalMethods, ObjectKind, ObjectMeta},
        string::JSString,
    },
    JSValue,
};

#[derive(Debug, Default, PartialEq)]
pub enum ThisBindingStatus {
    Lexical,
    Initialized,
    #[default]
    Uninitialized,
}

/// 9.1.1.2 Function Environment Records
/// https://262.ecma-international.org/16.0/#sec-function-environment-records
#[derive(Debug, Default)]
pub(crate) struct FuncEnvironment {
    /// [[OuterEnv]]
    pub(crate) outer_env: Option<EnvironmentAddr>,
    pub(crate) decl_env: DeclEnvironment,

    /// [[ThisValue]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    pub(crate) this_value: Option<JSValue>,

    /// [[ThisBindingStatus]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    pub(crate) this_binding_status: ThisBindingStatus,

    /// [[FunctionObject]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    pub(crate) function_object: Option<ObjectAddr>,

    /// [[NewTarget]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    pub(crate) new_target: Option<ObjectAddr>,
}

impl EnvironmentMethods for FuncEnvironment {
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        self.decl_env.has_binding(name)
    }

    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord {
        self.decl_env.create_mutable_binding(name, deletable)
    }

    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord {
        self.decl_env.create_immutable_binding(name, strict)
    }

    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord {
        self.decl_env.initialize_binding(name, value)
    }

    fn set_mutable_binding(
        &mut self,

        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        self.decl_env.set_mutable_binding(name, value, strict)
    }

    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue> {
        self.decl_env.get_binding_value(name, strict)
    }

    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        self.decl_env.delete_binding(name)
    }

    fn has_this_binding(&self) -> bool {
        self.decl_env.has_this_binding()
    }

    fn has_super_binding(&self) -> bool {
        self.decl_env.has_super_binding()
    }

    fn with_base_object(&self) -> Option<ObjectAddr> {
        self.decl_env.with_base_object()
    }
}

impl FuncEnvironment {
    /// 9.1.1.3.1 BindThisValue ( envRec, V )
    /// https://262.ecma-international.org/16.0/#sec-bindthisvalue
    pub(crate) fn bind_this_value(&mut self, value: JSValue) -> CompletionRecord {
        // 1. Assert: envRec.[[ThisBindingStatus]] is not lexical.
        debug_assert!(self.this_binding_status != ThisBindingStatus::Lexical);

        // 2. If envRec.[[ThisBindingStatus]] is initialized, throw a ReferenceError exception.
        if self.this_binding_status == ThisBindingStatus::Initialized {
            reference_error("Cannot bind 'this' value multiple times");
        }

        // 3. Set envRec.[[ThisValue]] to V.
        self.this_value = Some(value);

        // 4. Set envRec.[[ThisBindingStatus]] to initialized.
        self.this_binding_status = ThisBindingStatus::Initialized;

        // 5. Return unused.
        Ok(())
    }

    /// 9.1.1.3.2 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records-hasthisbinding
    pub(crate) fn has_this_binding(&self) -> bool {
        // 1. If envRec.[[ThisBindingStatus]] is lexical, return false; otherwise, return true.
        self.this_binding_status != ThisBindingStatus::Lexical
    }

    /// 9.1.1.3.3 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records-hassuperbinding
    pub(crate) fn has_super_binding(&self) -> bool {
        // 1. If envRec.[[ThisBindingStatus]] is lexical, return false.
        if self.this_binding_status == ThisBindingStatus::Lexical {
            return false;
        }

        // 2. If envRec.[[FunctionObject]].[[HomeObject]] is undefined, return false; otherwise, return true.
        if let Some(object) = self.function_object.clone() {
            return object.data().slots().home_object().is_some();
        }

        false
    }

    /// 9.1.1.3.4 GetThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records-getthisbinding
    pub(crate) fn get_this_binding(&self) -> CompletionRecord<JSValue> {
        // 1. Assert: envRec.[[ThisBindingStatus]] is not lexical.
        debug_assert!(self.this_binding_status != ThisBindingStatus::Lexical);

        // 2. If envRec.[[ThisBindingStatus]] is uninitialized, throw a ReferenceError exception.
        if self.this_binding_status == ThisBindingStatus::Uninitialized {
            reference_error("Cannot get 'this' value which is uninitialized");
        }

        // 3. Return envRec.[[ThisValue]].
        Ok(self.this_value.clone().unwrap_or(JSValue::Undefined))
    }

    /// 9.1.1.3.5 GetSuperBase ( envRec )
    /// https://262.ecma-international.org/16.0/#sec-getsuperbase
    pub(crate) fn get_super_base(&self) -> Option<ObjectAddr> {
        // 1. Let home be envRec.[[FunctionObject]].[[HomeObject]].
        let Some(function_object) = self.function_object.clone() else {
            // 2. If home is undefined, return undefined.
            return None;
        };

        let Some(home) = function_object.data().slots().home_object() else {
            // 2. If home is undefined, return undefined.
            return None;
        };

        // 3. Assert: home is an ordinary object.
        assert!(home.kind() == ObjectKind::Ordinary);

        // 4. Return ! home.[[GetPrototypeOf]]().
        home.get_prototype_of()
    }
}
