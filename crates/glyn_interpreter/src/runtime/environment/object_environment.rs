use crate::{
    abstract_ops::{
        object_operations::{define_property_or_throw, get, has_property},
        type_conversion::to_boolean,
    },
    runtime::{
        completion::CompletionRecord,
        environment::{EnvironmentAddr, EnvironmentMethods},
    },
    value::{
        object::{
            property::{JSObjectPropDescriptor, JSObjectPropKey},
            JSObjAddr,
        },
        string::JSString,
    },
    JSAgent, JSValue,
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
    /// 9.1.1.2.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasbinding-n
    pub(crate) fn has_binding(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool> {
        let obj_env = agent.environment(env_addr).obj_env();

        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object_addr = obj_env.binding_object.unwrap();

        // 2. Let foundBinding be ? HasProperty(bindingObject, N).
        let found_binding = has_property(agent, binding_object_addr, &JSObjectPropKey::from(name));

        // 3. If foundBinding is false, return false.
        if !found_binding {
            return Ok(false);
        }

        // 4. If envRec.[[IsWithEnvironment]] is false, return true.
        if !obj_env.is_with_environment {
            return Ok(true);
        }

        // 5. Let unscopables be ? Get(bindingObject, %Symbol.unscopables%).
        let unscopables = get(
            agent,
            binding_object_addr,
            &JSObjectPropKey::from(&agent.well_known_symbols().unscopables),
            &JSValue::from(binding_object_addr),
        )?;

        // 6. If unscopables is an Object, then
        if let Some(unscopables_obj) = unscopables.as_object() {
            // a. Let blocked be ToBoolean(? Get(unscopables, N)).
            let blocked = to_boolean(
                agent,
                get(
                    agent,
                    unscopables_obj,
                    &JSObjectPropKey::from(name),
                    &JSValue::from(unscopables_obj),
                )?,
            );

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
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        configurable: bool,
    ) -> CompletionRecord {
        let obj_env = agent.environment(env_addr).obj_env();

        // 1. Let bindingObject be envRec.[[BindingObject]].
        let binding_object = obj_env.binding_object.unwrap();

        // 2. Perform ? DefinePropertyOrThrow(bindingObject, N, PropertyDescriptor { [[Value]]: undefined, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: D }).
        define_property_or_throw(
            agent,
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
        _agent: &mut JSAgent,
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
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
    ) -> CompletionRecord {
        // 1. Perform ? envRec.SetMutableBinding(N, V, false).
        ObjEnvironment::set_mutable_binding(agent, env_addr, name, value, false)?;

        // 2. Return unused.
        Ok(())
    }

    /// 9.1.1.2.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-setmutablebinding-n-v-s
    pub(crate) fn set_mutable_binding(
        _agent: &mut JSAgent,
        _env_addr: EnvironmentAddr,
        _name: JSString,
        _value: JSValue,
        _strict: bool,
    ) -> CompletionRecord {
        todo!()
    }

    /// 9.1.1.2.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-getbindingvalue-n-s
    pub(crate) fn get_binding_value(
        _agent: &JSAgent,
        _env_addr: EnvironmentAddr,
        _name: &JSString,
        _strict: bool,
    ) -> CompletionRecord<JSValue> {
        todo!()
    }

    /// 9.1.1.2.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-deletebinding-n
    pub(crate) fn delete_binding(
        _agent: &mut JSAgent,
        _env_addr: EnvironmentAddr,
        _name: &JSString,
    ) -> CompletionRecord<bool> {
        todo!()
    }

    /// 9.1.1.2.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hasthisbinding
    pub(crate) fn has_this_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        todo!()
    }

    /// 9.1.1.2.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-hassuperbinding
    pub(crate) fn has_super_binding(_agent: &JSAgent, _env_addr: EnvironmentAddr) -> bool {
        todo!()
    }

    /// 9.1.1.2.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records-withbaseobject
    pub(crate) fn with_base_object(
        _agent: &JSAgent,
        _env_addr: EnvironmentAddr,
    ) -> Option<JSObjAddr> {
        todo!()
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
