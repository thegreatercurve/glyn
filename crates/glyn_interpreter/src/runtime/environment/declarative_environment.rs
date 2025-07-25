use std::collections::HashMap;

use crate::{
    runtime::{
        agent::{reference_error, type_error},
        completion::{throw_completion, CompletionRecord, ThrowCompletion},
        environment::{Environment, EnvironmentAddr, EnvironmentMethods},
    },
    value::{object::ObjectAddr, string::JSString},
    JSValue,
};

#[derive(Clone, Debug)]
pub(crate) struct Binding {
    value: Option<JSValue>,
    mutable: bool,
    deletable: bool,
    strict: bool,
}

/// 9.1.1.1 Declarative Environment Records
/// https://262.ecma-international.org/16.0/#sec-declarative-environment-records
#[derive(Debug, Default)]
pub(crate) struct DeclarativeEnvironment {
    /// [[OuterEnv]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-declarative-environment-records
    pub(crate) outer_env: Option<EnvironmentAddr>,

    bindings: HashMap<JSString, Binding>,
}

impl DeclarativeEnvironment {
    fn binding(&self, name: &JSString) -> &Binding {
        self.bindings.get(name).unwrap()
    }

    fn binding_mut(&mut self, name: &JSString) -> &mut Binding {
        self.bindings.get_mut(name).unwrap()
    }

    fn has_binding_impl(&self, name: &JSString) -> bool {
        self.bindings.contains_key(name)
    }

    fn add_binding_impl(&mut self, name: JSString, mutable: bool, deletable: bool, strict: bool) {
        debug_assert!(!self.has_binding_impl(&name));

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

    fn initialize_binding_impl(&mut self, name: JSString, value: JSValue) {
        debug_assert!(self.binding(&name).value.is_none());

        self.binding_mut(&name).value = Some(value);
    }

    fn remove_binding_impl(&mut self, name: &JSString) {
        self.bindings.remove(name);
    }
}

impl EnvironmentMethods for DeclarativeEnvironment {
    /// 9.1.1.1.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hasbinding-n
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. If envRec has a binding for N, return true.
        // 2. Return false.
        Ok(self.has_binding_impl(name))
    }

    /// 9.1.1.1.2 CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-createmutablebinding-n-d
    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord {
        // 1. Assert: envRec does not already have a binding for N.
        // 2. Create a mutable binding in envRec for N and record that it is uninitialized. If D is true, record that the newly created binding may be deleted by a subsequent DeleteBinding call.
        self.add_binding_impl(name, true, deletable, true);

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.1.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-createimmutablebinding-n-s
    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord {
        // 1. Assert: envRec does not already have a binding for N.
        // Create an immutable binding in envRec for N and record that it is uninitialized. If S is true, record that the newly created binding is a strict binding.
        self.add_binding_impl(name, false, false, strict);

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.1.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-initializebinding-n-v
    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord {
        // 1. Assert: envRec must have an uninitialized binding for N.
        // 2. Set the bound value for N in envRec to V.
        self.initialize_binding_impl(name, value);

        // 3. Record that the binding for N in envRec has been initialized.
        // Note: This is implicit in setting the value to Some(value)

        // 4. Return unused.
        Ok(())
    }

    /// 9.1.1.1.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-setmutablebinding-n-v-s
    fn set_mutable_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        mut strict: bool,
    ) -> CompletionRecord {
        // 1. If envRec does not have a binding for N, then
        if !self.has_binding_impl(&name) {
            // a. If S is true, throw a ReferenceError exception.
            if strict {
                reference_error(&format!("Property {name:?} is not defined"));
            }

            // b. Perform ! envRec.CreateMutableBinding(N, true).
            self.add_binding_impl(name.clone(), true, true, true);

            // c. Perform ! envRec.InitializeBinding(N, V).
            self.initialize_binding_impl(name, value);

            // d. Return unused.
            return Ok(());
        }

        // 2. If the binding for N in envRec is a strict binding, set S to true.
        if self.binding(&name).strict {
            strict = true;
        }

        // 3. If the binding for N in envRec has not yet been initialized, then
        if self.binding(&name).value.is_none() {
            // a. Throw a ReferenceError exception.
            reference_error(&format!("Property {name:?} is not defined"));
        }
        // 4. Else if the binding for N in envRec is a mutable binding, then
        else if self.binding(&name).mutable {
            // a. Change its bound value to V.
            self.binding_mut(&name).value = Some(value);
        }
        // 5. Else,
        else {
            // a. Assert: This is an attempt to change the value of an immutable binding.
            // b. If S is true, throw a TypeError exception.
            if strict {
                type_error(&format!(
                    "Cannot change the value of an immutable property: {name:?}"
                ));
            }
        }

        // 6. Return unused.
        Ok(())
    }

    /// 9.1.1.1.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-getbindingvalue-n-s
    fn get_binding_value(&self, name: &JSString, _strict: bool) -> CompletionRecord<JSValue> {
        // 1. Assert: envRec has a binding for N.
        debug_assert!(self.has_binding_impl(name));

        // 2. If the binding for N in envRec is an uninitialized binding, throw a ReferenceError exception.
        // 3. Return the value currently bound to N in envRec.
        if let Some(value) = &self.binding(name).value {
            Ok(value.clone())
        } else {
            reference_error(&format!("Property {name:?} is not initialized"))
        }
    }

    /// 9.1.1.1.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-deletebinding-n
    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Assert: envRec has a binding for N.
        debug_assert!(self.has_binding_impl(name));

        // 2. If the binding for N in envRec cannot be deleted, return false.
        if !self.binding(name).deletable {
            return Ok(false);
        }

        // 3. Remove the binding for N from envRec.
        self.remove_binding_impl(name);

        // 4. Return true.
        Ok(true)
    }

    /// 9.1.1.1.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hasthisbinding
    fn has_this_binding(&self) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.1.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-hassuperbinding
    fn has_super_binding(&self) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.1.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records-withbaseobject
    fn with_base_object(&self) -> Option<ObjectAddr> {
        // 1. Return undefined.
        None
    }
}

impl<'a> TryFrom<&'a mut Environment> for &'a mut DeclarativeEnvironment {
    type Error = ThrowCompletion;

    fn try_from(value: &'a mut Environment) -> Result<&'a mut DeclarativeEnvironment, Self::Error> {
        match value {
            Environment::Declarative(decl_env) => Ok(decl_env),
            _ => throw_completion(
                "Expected Environment::Declarative for conversion to DeclarativeEnvironment",
            ),
        }
    }
}
