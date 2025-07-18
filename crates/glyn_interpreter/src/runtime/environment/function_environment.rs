use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclEnvironment, EnvironmentAddr, EnvironmentMethods,
        },
    },
    value::{object::JSObjAddr, string::JSString},
    JSValue,
};

#[derive(Debug, Default)]
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
    pub(crate) function_object: Option<JSObjAddr>,

    /// [[NewTarget]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    pub(crate) new_target: Option<JSObjAddr>,
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

    fn with_base_object(&self) -> Option<JSObjAddr> {
        self.decl_env.with_base_object()
    }
}
