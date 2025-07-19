use crate::{
    abstract_ops::{
        object_operations::{define_property_or_throw, get, has_property, set},
        type_conversion::to_boolean,
    },
    runtime::{
        agent::{reference_error, WELL_KNOWN_SYMBOLS_UNSCOPABLES},
        completion::CompletionRecord,
        environment::{EnvironmentAddr, EnvironmentMethods},
    },
    value::{
        object::{
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            JSObjAddr, JSObjectInternalMethods,
        },
        string::JSString,
    },
    JSValue,
};

/// 9.1.1.2 Object Environment Records
/// https://262.ecma-international.org/16.0/#sec-object-environment-records
#[derive(Debug, Default)]
pub(crate) struct ObjEnvironment {
    /// [[BindingObject]]
    pub(crate) binding_object: Option<JSObjAddr>,

    /// [[IsWithEnvironment]]
    pub(crate) is_with_environment: bool,
}

impl ObjEnvironment {
    pub(crate) fn binding_object(&self) -> JSObjAddr {
        self.binding_object.clone().unwrap()
    }
}

impl ObjEnvironment {
    /// 9.1.1.2.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasbinding-n
    pub(crate) fn has_binding(
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object_addr = env_addr.borrow().obj_env().binding_object();

        // 2. Let foundBinding be ? HasProperty(bindingObject, N).
        let found_binding =
            has_property(binding_object_addr.clone(), &JSObjectPropKey::from(name))?;

        // 3. If foundBinding is false, return false.
        if !found_binding {
            return Ok(false);
        }

        // 4. If envRec.[[IsWithEnvironment]] is false, return true.
        if !env_addr.borrow().obj_env().is_with_environment {
            return Ok(true);
        }

        // 5. Let unscopables be ? Get(bindingObject, %Symbol.unscopables%).
        let unscopables = get(
            binding_object_addr.clone(),
            &JSObjectPropKey::from(WELL_KNOWN_SYMBOLS_UNSCOPABLES),
            &JSValue::from(binding_object_addr.clone()),
        )?;

        // 6. If unscopables is an Object, then
        if let Some(unscopables_obj) = unscopables.as_object() {
            // a. Let blocked be ToBoolean(? Get(unscopables, N)).
            let blocked = to_boolean(get(
                unscopables_obj.clone(),
                &JSObjectPropKey::from(name),
                &JSValue::from(unscopables_obj),
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
    pub(crate) fn create_mutable_binding(
        env_addr: EnvironmentAddr,
        name: JSString,
        configurable: bool,
    ) -> CompletionRecord {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = env_addr.borrow().obj_env().binding_object();

        // 2. Perform ? DefinePropertyOrThrow(bindingObject, N, PropertyDescriptor { [[Value]]: undefined, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: D }).
        define_property_or_throw(
            binding_object,
            &JSObjectPropKey::from(name),
            JSObjectPropDescriptor {
                value: None,
                writable: Some(true),
                enumerable: Some(true),
                configurable: Some(configurable),
                ..JSObjectPropDescriptor::default()
            },
        )?;

        // 3. Return unused.
        Ok(())
    }

    /// 9.1.1.2.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-createimmutablebinding-n-s
    pub(crate) fn create_immutable_binding(
        _env_addr: EnvironmentAddr,
        _name: JSString,
        _strict: bool,
    ) -> CompletionRecord {
        // The CreateImmutableBinding concrete method of an Object Environment Record is never used within this specification.
        unreachable!()
    }

    /// 9.1.1.2.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-initializebinding-n-v
    pub(crate) fn initialize_binding(
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
    ) -> CompletionRecord {
        // 1. Perform ? envRec.SetMutableBinding(N, V, false).
        ObjEnvironment::set_mutable_binding(env_addr, name, value, false)?;

        // 2. Return unused.
        Ok(())
    }

    /// 9.1.1.2.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-setmutablebinding-n-v-s
    pub(crate) fn set_mutable_binding(
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = env_addr.borrow().obj_env().binding_object();

        // 2. Let stillExists be ? HasProperty(bindingObject, N).
        let still_exists = has_property(binding_object.clone(), &JSObjectPropKey::from(&name))?;

        // 3. If stillExists is false and S is true, throw a ReferenceError exception.
        if !still_exists && strict {
            reference_error(&format!("Property {name:?} is not defined"));
        }

        // 4. Perform ? Set(bindingObject, N, V, S).
        set(binding_object, &JSObjectPropKey::from(name), value, strict)?;

        // 5. Return unused.
        Ok(())
    }

    /// 9.1.1.2.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-getbindingvalue-n-s
    pub(crate) fn get_binding_value(
        env_addr: EnvironmentAddr,
        name: &JSString,
        strict: bool,
    ) -> CompletionRecord<JSValue> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = env_addr.borrow().obj_env().binding_object();

        // 2. Let value be ? HasProperty(bindingObject, N).
        let value = has_property(binding_object.clone(), &JSObjectPropKey::from(name))?;

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
            binding_object.clone(),
            &JSObjectPropKey::from(name),
            &JSValue::from(binding_object),
        )
    }

    /// 9.1.1.2.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-deletebinding-n
    pub(crate) fn delete_binding(
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = env_addr.borrow().obj_env().binding_object();

        // 2. Return ? bindingObject.[[Delete]](N).
        binding_object.delete(&JSObjectPropKey::from(name))
    }

    /// 9.1.1.2.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasthisbinding
    pub(crate) fn has_this_binding(_env_addr: EnvironmentAddr) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.2.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hassuperbinding
    pub(crate) fn has_super_binding(_env_addr: EnvironmentAddr) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.2.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-withbaseobject
    pub(crate) fn with_base_object(env_addr: EnvironmentAddr) -> Option<JSObjAddr> {
        // 1. If envRec.[[IsWithEnvironment]] is true, return envRec.[[BindingObject]].
        if env_addr.borrow().obj_env().is_with_environment {
            return Some(env_addr.borrow().obj_env().binding_object());
        }

        // 2. Otherwise, return undefined.
        None
    }
}

pub(crate) static OBJECT_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: ObjEnvironment::has_binding,
    create_mutable_binding: ObjEnvironment::create_mutable_binding,
    create_immutable_binding: ObjEnvironment::create_immutable_binding,
    initialize_binding: ObjEnvironment::initialize_binding,
    set_mutable_binding: ObjEnvironment::set_mutable_binding,
    get_binding_value: ObjEnvironment::get_binding_value,
    delete_binding: ObjEnvironment::delete_binding,
    has_this_binding: ObjEnvironment::has_this_binding,
    has_super_binding: ObjEnvironment::has_super_binding,
    with_base_object: ObjEnvironment::with_base_object,
};
