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
            declarative_environment::DeclEnvironment, function_environment::FuncEnvironment,
            global_environment::GlobalEnvironment, object_environment::ObjEnvironment,
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
    Declarative(DeclEnvironment),

    /// 9.1.1.2 Object Environment Records
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records
    Object(ObjEnvironment),

    /// 9.1.1.3 Function Environment Records
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records
    Function(FuncEnvironment),

    /// 9.1.1.4 Global Environment Records
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records
    Global(GlobalEnvironment),
}

pub(crate) type EnvironmentAddr = Gc<Environment>;

impl EnvironmentAddr {
    pub(crate) fn outer(&self) -> Option<EnvironmentAddr> {
        match self.borrow().deref() {
            Environment::Declarative(decl_environment) => decl_environment.outer_env.clone(),
            Environment::Object(obj_environment) => obj_environment.outer_env.clone(),
            Environment::Function(func_environment) => func_environment.outer_env.clone(),
            Environment::Global(global_environment) => global_environment.outer_env.clone(),
        }
    }
}

impl EnvironmentMethods for EnvironmentAddr {
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        match self.borrow().deref() {
            Environment::Declarative(decl_environment) => decl_environment.has_binding(name),
            Environment::Object(obj_environment) => obj_environment.has_binding(name),
            Environment::Function(func_environment) => func_environment.has_binding(name),
            Environment::Global(global_environment) => global_environment.has_binding(name),
        }
    }

    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(decl_environment) => {
                decl_environment.create_mutable_binding(name, deletable)
            }
            Environment::Object(obj_environment) => {
                obj_environment.create_mutable_binding(name, deletable)
            }
            Environment::Function(func_environment) => {
                func_environment.create_mutable_binding(name, deletable)
            }
            Environment::Global(global_environment) => {
                global_environment.create_mutable_binding(name, deletable)
            }
        }
    }

    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(decl_environment) => {
                decl_environment.create_immutable_binding(name, strict)
            }
            Environment::Object(obj_environment) => {
                obj_environment.create_immutable_binding(name, strict)
            }
            Environment::Function(func_environment) => {
                func_environment.create_immutable_binding(name, strict)
            }
            Environment::Global(global_environment) => {
                global_environment.create_immutable_binding(name, strict)
            }
        }
    }

    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(decl_environment) => {
                decl_environment.initialize_binding(name, value)
            }
            Environment::Object(obj_environment) => obj_environment.initialize_binding(name, value),
            Environment::Function(func_environment) => {
                func_environment.initialize_binding(name, value)
            }
            Environment::Global(global_environment) => {
                global_environment.initialize_binding(name, value)
            }
        }
    }

    fn set_mutable_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(decl_environment) => {
                decl_environment.set_mutable_binding(name, value, strict)
            }
            Environment::Object(obj_environment) => {
                obj_environment.set_mutable_binding(name, value, strict)
            }
            Environment::Function(func_environment) => {
                func_environment.set_mutable_binding(name, value, strict)
            }
            Environment::Global(global_environment) => {
                global_environment.set_mutable_binding(name, value, strict)
            }
        }
    }

    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue> {
        match self.borrow().deref() {
            Environment::Declarative(decl_environment) => {
                decl_environment.get_binding_value(name, strict)
            }
            Environment::Object(obj_environment) => obj_environment.get_binding_value(name, strict),
            Environment::Function(func_environment) => {
                func_environment.get_binding_value(name, strict)
            }
            Environment::Global(global_environment) => {
                global_environment.get_binding_value(name, strict)
            }
        }
    }

    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        match self.borrow_mut().deref_mut() {
            Environment::Declarative(decl_environment) => decl_environment.delete_binding(name),
            Environment::Object(obj_environment) => obj_environment.delete_binding(name),
            Environment::Function(func_environment) => func_environment.delete_binding(name),
            Environment::Global(global_environment) => global_environment.delete_binding(name),
        }
    }

    fn has_this_binding(&self) -> bool {
        match self.borrow().deref() {
            Environment::Declarative(decl_environment) => decl_environment.has_this_binding(),
            Environment::Object(obj_environment) => obj_environment.has_this_binding(),
            Environment::Function(func_environment) => func_environment.has_this_binding(),
            Environment::Global(global_environment) => global_environment.has_this_binding(),
        }
    }

    fn has_super_binding(&self) -> bool {
        match self.borrow().deref() {
            Environment::Declarative(decl_environment) => decl_environment.has_super_binding(),
            Environment::Object(obj_environment) => obj_environment.has_super_binding(),
            Environment::Function(func_environment) => func_environment.has_super_binding(),
            Environment::Global(global_environment) => global_environment.has_super_binding(),
        }
    }

    fn with_base_object(&self) -> Option<ObjectAddr> {
        todo!()
    }
}
