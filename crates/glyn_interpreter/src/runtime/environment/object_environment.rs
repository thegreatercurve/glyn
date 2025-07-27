use crate::{
    abstract_ops::{
        object_operations::{define_property_or_throw, get, has_property, set},
        type_conversion::to_boolean,
    },
    runtime::{
        agent::{reference_error, WELL_KNOWN_SYMBOLS_UNSCOPABLES},
        completion::{throw_completion, CompletionRecord, ThrowCompletion},
        environment::{Environment, EnvironmentAddr, EnvironmentMethods},
    },
    value::{
        object::{
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            ObjectAddr, ObjectEssentialInternalMethods,
        },
        string::JSString,
    },
    JSValue,
};

/// 9.1.1.2 Object Environment Records
/// https://262.ecma-international.org/16.0/#sec-object-environment-records
#[derive(Clone, Debug)]
pub(crate) struct ObjectEnvironment {
    /// [[OuterEnv]]
    pub(crate) outer_env: Option<EnvironmentAddr>,

    /// [[BindingObject]]
    pub(crate) binding_object: ObjectAddr,

    /// [[IsWithEnvironment]]
    pub(crate) is_with_environment: bool,
}

impl EnvironmentMethods for ObjectEnvironment {
    /// 9.1.1.2.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasbinding-n
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = self.binding_object.clone();

        // 2. Let foundBinding be ? HasProperty(bindingObject, N).
        let found_binding = has_property(&binding_object, &JSObjectPropKey::from(name))?;

        // 3. If foundBinding is false, return false.
        if !found_binding {
            return Ok(false);
        }

        // 4. If envRec.[[IsWithEnvironment]] is false, return true.
        if !self.is_with_environment {
            return Ok(true);
        }

        // 5. Let unscopables be ? Get(bindingObject, %Symbol.unscopables%).
        let unscopables = get(
            &binding_object,
            &JSObjectPropKey::from(WELL_KNOWN_SYMBOLS_UNSCOPABLES),
            &JSValue::from(self.binding_object.clone()),
        )?;

        // 6. If unscopables is an Object, then
        if let Ok(unscopables_obj) = ObjectAddr::try_from(unscopables) {
            // a. Let blocked be ToBoolean(? Get(unscopables, N)).
            let blocked = to_boolean(get(
                &unscopables_obj,
                &JSObjectPropKey::from(name),
                &JSValue::from(unscopables_obj.clone()),
            )?);

            // b. If blocked is true, return false.
            if blocked {
                return Ok(false);
            }
        }

        // 7. Return true.
        Ok(true)
    }

    /// 9.1.1.2.2 CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-createmutablebinding-n-d
    fn create_mutable_binding(&mut self, name: &JSString, deletable: bool) -> CompletionRecord {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = self.binding_object.clone();

        // 2. Perform ? DefinePropertyOrThrow(bindingObject, N, PropertyDescriptor { [[Value]]: undefined, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: D }).
        define_property_or_throw(
            &binding_object,
            &JSObjectPropKey::from(name),
            JSObjectPropDescriptor {
                value: None,
                writable: Some(true),
                enumerable: Some(true),
                configurable: Some(deletable),
                ..JSObjectPropDescriptor::default()
            },
        )?;

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.2.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-createimmutablebinding-n-s
    fn create_immutable_binding(&mut self, name: &JSString, strict: bool) -> CompletionRecord {
        // The CreateImmutableBinding concrete method of an Object Environment Record is never used within this specification.
        unreachable!()
    }

    /// 9.1.1.2.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-initializebinding-n-v
    fn initialize_binding(&mut self, name: &JSString, value: JSValue) -> CompletionRecord {
        // 1. Perform ? envRec.SetMutableBinding(N, V, false).
        self.set_mutable_binding(name, value, false)?;

        // 2. Return unused.
        Ok(())
    }

    /// 9.1.1.2.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-setmutablebinding-n-v-s
    fn set_mutable_binding(
        &mut self,

        name: &JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = self.binding_object.clone();

        // 2. Let stillExists be ? HasProperty(bindingObject, N).
        let still_exists = has_property(&binding_object, &JSObjectPropKey::from(name))?;

        // 3. If stillExists is false and S is true, throw a ReferenceError exception.
        if !still_exists && strict {
            reference_error(&format!("Property {name:?} is not defined"));
        }

        // 4. Perform ? Set(bindingObject, N, V, S).
        set(&binding_object, &JSObjectPropKey::from(name), value, strict)?;

        // 5. Return unused.
        Ok(())
    }

    /// 9.1.1.2.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-getbindingvalue-n-s
    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = self.binding_object.clone();

        // 2. Let value be ? HasProperty(bindingObject, N).
        let value = has_property(&binding_object, &JSObjectPropKey::from(name))?;

        // 3. If value is false, then
        if !value {
            // a. If S is false, return undefined; otherwise throw a ReferenceError exception.
            if strict {
                reference_error(&format!("Property {name:?} is not defined"));
            }

            return Ok(JSValue::Undefined);
        }

        // 4. Return ? Get(bindingObject, N).
        get(
            &binding_object,
            &JSObjectPropKey::from(name),
            &JSValue::from(self.binding_object.clone()),
        )
    }

    /// 9.1.1.2.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-deletebinding-n
    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = self.binding_object.clone();

        // 2. Return ? bindingObject.[[Delete]](N).
        binding_object.delete(&JSObjectPropKey::from(name))
    }

    /// 9.1.1.2.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasthisbinding
    fn has_this_binding(&self) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.2.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hassuperbinding
    fn has_super_binding(&self) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.2.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-withbaseobject
    fn with_base_object(&self) -> Option<ObjectAddr> {
        // 1. If envRec.[[IsWithEnvironment]] is true, return envRec.[[BindingObject]].
        if self.is_with_environment {
            return Some(self.binding_object.clone());
        }

        // 2. Otherwise, return undefined.
        None
    }
}

impl<'a> TryFrom<&'a mut Environment> for &'a mut ObjectEnvironment {
    type Error = ThrowCompletion;

    fn try_from(value: &'a mut Environment) -> Result<&'a mut ObjectEnvironment, Self::Error> {
        match value {
            Environment::Object(object_env) => Ok(object_env),
            _ => {
                throw_completion("Expected Environment::Object for conversion to ObjectEnvironment")
            }
        }
    }
}
