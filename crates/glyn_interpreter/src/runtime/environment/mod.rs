pub(crate) mod declarative_environment;
pub(crate) mod function_environment;
pub(crate) mod global_environment;
pub(crate) mod object_environment;

use std::ops::{Deref, DerefMut};

use crate::{
    gc::Gc,
    runtime::{
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclarativeEnvironment,
            function_environment::FunctionEnvironment, global_environment::GlobalEnvironment,
            object_environment::ObjectEnvironment,
        },
    },
    value::{object::ObjectAddr, string::JSString, JSValue},
};

pub(crate) trait EnvironmentMethods {
    /// HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool>;

    /// CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord;

    /// CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord;

    /// InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord;

    /// SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn set_mutable_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord;

    /// GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue>;

    /// DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool>;

    /// HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn has_this_binding(&self) -> bool;

    /// HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn has_super_binding(&self) -> bool;

    /// WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    fn with_base_object(&self) -> Option<ObjectAddr>;
}

/// 9.1.1 The Environment Record Type Hierarchy
/// https://262.ecma-international.org/16.0/#sec-the-environment-record-type-hierarchy
#[derive(Debug)]
pub(crate) enum Environment {
    /// 9.1.1.1 Declarative Environment Records
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records
    Declarative(DeclarativeEnvironment),

    /// 9.1.1.2 Object Environment Records
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records
    Object(ObjectEnvironment),

    /// 9.1.1.3 Function Environment Records
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records
    Function(FunctionEnvironment),

    /// 9.1.1.4 Global Environment Records
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records
    Global(GlobalEnvironment),
}

pub(crate) type EnvironmentAddr = Gc<Environment>;

impl EnvironmentAddr {
    pub(crate) fn outer(&self) -> Option<EnvironmentAddr> {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => declarative_env.outer_env.clone(),
            Environment::Object(object_env) => object_env.outer_env.clone(),
            Environment::Function(function_env) => function_env.outer_env.clone(),
            Environment::Global(global_env) => global_env.outer_env.clone(),
        }
    }
}

impl EnvironmentMethods for EnvironmentAddr {
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => declarative_env.has_binding(name),
            Environment::Object(object_env) => object_env.has_binding(name),
            Environment::Function(function_env) => function_env.has_binding(name),
            Environment::Global(global_env) => global_env.has_binding(name),
        }
    }

    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(declarative_env) => {
                declarative_env.create_mutable_binding(name, deletable)
            }
            Environment::Object(object_env) => object_env.create_mutable_binding(name, deletable),
            Environment::Function(function_env) => {
                function_env.create_mutable_binding(name, deletable)
            }
            Environment::Global(global_env) => global_env.create_mutable_binding(name, deletable),
        }
    }

    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(declarative_env) => {
                declarative_env.create_immutable_binding(name, strict)
            }
            Environment::Object(object_env) => object_env.create_immutable_binding(name, strict),
            Environment::Function(function_env) => {
                function_env.create_immutable_binding(name, strict)
            }
            Environment::Global(global_env) => global_env.create_immutable_binding(name, strict),
        }
    }

    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(declarative_env) => {
                declarative_env.initialize_binding(name, value)
            }
            Environment::Object(object_env) => object_env.initialize_binding(name, value),
            Environment::Function(function_env) => function_env.initialize_binding(name, value),
            Environment::Global(global_env) => global_env.initialize_binding(name, value),
        }
    }

    fn set_mutable_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(declarative_env) => {
                declarative_env.set_mutable_binding(name, value, strict)
            }
            Environment::Object(object_env) => object_env.set_mutable_binding(name, value, strict),
            Environment::Function(function_env) => {
                function_env.set_mutable_binding(name, value, strict)
            }
            Environment::Global(global_env) => global_env.set_mutable_binding(name, value, strict),
        }
    }

    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue> {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => {
                declarative_env.get_binding_value(name, strict)
            }
            Environment::Object(object_env) => object_env.get_binding_value(name, strict),
            Environment::Function(function_env) => function_env.get_binding_value(name, strict),
            Environment::Global(global_env) => global_env.get_binding_value(name, strict),
        }
    }

    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(declarative_env) => declarative_env.delete_binding(name),
            Environment::Object(object_env) => object_env.delete_binding(name),
            Environment::Function(function_env) => function_env.delete_binding(name),
            Environment::Global(global_env) => global_env.delete_binding(name),
        }
    }

    fn has_this_binding(&self) -> bool {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => declarative_env.has_this_binding(),
            Environment::Object(object_env) => object_env.has_this_binding(),
            Environment::Function(function_env) => function_env.has_this_binding(),
            Environment::Global(global_env) => global_env.has_this_binding(),
        }
    }

    fn has_super_binding(&self) -> bool {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => declarative_env.has_super_binding(),
            Environment::Object(object_env) => object_env.has_super_binding(),
            Environment::Function(function_env) => function_env.has_super_binding(),
            Environment::Global(global_env) => global_env.has_super_binding(),
        }
    }

    fn with_base_object(&self) -> Option<ObjectAddr> {
        match self.borrow().deref() {
            Environment::Declarative(declarative_env) => declarative_env.with_base_object(),
            Environment::Object(object_env) => object_env.with_base_object(),
            Environment::Function(function_env) => function_env.with_base_object(),
            Environment::Global(global_env) => global_env.with_base_object(),
        }
    }
}
