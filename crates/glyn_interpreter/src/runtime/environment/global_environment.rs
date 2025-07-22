use crate::{
    abstract_ops::{
        object_operations::{define_property_or_throw, has_own_property, has_property, set},
        testing_comparison::is_extensible,
    },
    runtime::{
        agent::type_error,
        completion::CompletionRecord,
        environment::{
            declarative_environment::DeclEnvironment, object_environment::ObjEnvironment,
            EnvironmentAddr, EnvironmentMethods,
        },
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

/// 9.1.1.4 Global Environment Records
/// https://262.ecma-international.org/16.0/#sec-global-environment-records
#[derive(Debug)]
pub(crate) struct GlobalEnvironment {
    /// [[OuterEnv]]
    pub(crate) outer_env: Option<EnvironmentAddr>,

    /// [[DeclarativeRecord]]
    pub(crate) declarative_record: DeclEnvironment,

    /// [[ObjectRecord]]
    pub(crate) object_record: ObjEnvironment,

    /// [[GlobalThisValue]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-global-environment-records
    pub(crate) global_this_value: Option<ObjectAddr>,
}

impl EnvironmentMethods for GlobalEnvironment {
    /// 9.1.1.4.1 HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hasbinding-n
    fn has_binding(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, return true.
        if self.declarative_record.has_binding(name)? {
            return Ok(true);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.HasBinding(N).
        self.object_record.has_binding(name)
    }

    /// 9.1.1.4.2 CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-createmutablebinding-n-d
    fn create_mutable_binding(&mut self, name: JSString, deletable: bool) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, throw a TypeError exception.
        if self.declarative_record.has_binding(&name)? {
            type_error(&format!("Binding already exists for {name:?}"));
        }

        // 3. Return ! DclRec.CreateMutableBinding(N, D).
        self.declarative_record
            .create_mutable_binding(name, deletable)
    }

    /// 9.1.1.4.3 CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-createimmutablebinding-n-s
    fn create_immutable_binding(&mut self, name: JSString, strict: bool) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, throw a TypeError exception.
        if self.declarative_record.has_binding(&name)? {
            type_error(&format!("Binding already exists for {name:?}"));
        }

        // 3. Return ! DclRec.CreateImmutableBinding(N, S).
        self.declarative_record
            .create_immutable_binding(name, strict)
    }

    /// 9.1.1.4.4 InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-initializebinding-n-v
    fn initialize_binding(&mut self, name: JSString, value: JSValue) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if self.declarative_record.has_binding(&name)? {
            // a. Return ! DclRec.InitializeBinding(N, V).
            return self.declarative_record.initialize_binding(name, value);
        }

        // 3. Assert: If the binding exists, it must be in the Object Environment Record.
        debug_assert!(self.object_record.has_binding(&name)?);

        // 4. Let ObjRec be envRec.[[ObjectRecord]].
        // 5. Return ? ObjRec.InitializeBinding(N, V).
        self.object_record.initialize_binding(name, value)
    }

    /// 9.1.1.4.5 SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-setmutablebinding-n-v-s
    fn set_mutable_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if self.declarative_record.has_binding(&name)? {
            // a. Return ? DclRec.SetMutableBinding(N, V, S).
            return self
                .declarative_record
                .set_mutable_binding(name, value, strict);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.SetMutableBinding(N, V, S).
        self.object_record.set_mutable_binding(name, value, strict)
    }

    /// 9.1.1.4.6 GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-getbindingvalue-n-s
    fn get_binding_value(&self, name: &JSString, strict: bool) -> CompletionRecord<JSValue> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if self.declarative_record.has_binding(name)? {
            // a. Return ? DclRec.GetBindingValue(N, S).
            return self.declarative_record.get_binding_value(name, strict);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Return ? ObjRec.GetBindingValue(N, S).
        self.object_record.get_binding_value(name, strict)
    }

    /// 9.1.1.4.7 DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-deletebinding-n
    fn delete_binding(&mut self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        // 2. If ! DclRec.HasBinding(N) is true, then
        if self.declarative_record.has_binding(name)? {
            // a. Return ! DclRec.DeleteBinding(N).
            return self.declarative_record.delete_binding(name);
        }

        // 3. Let ObjRec be envRec.[[ObjectRecord]].
        // 4. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = self.object_record.binding_object.clone();

        // 5. Let existingProp be ? HasOwnProperty(globalObject, N).
        let existing_prop = has_property(&global_object, &JSObjectPropKey::from(name))?;

        // 6. If existingProp is true, then
        if existing_prop {
            // a. Return ? ObjRec.DeleteBinding(N).
            return self.object_record.delete_binding(name);
        }

        // 7. Return true.
        Ok(true)
    }

    /// 9.1.1.4.8 HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hasthisbinding
    fn has_this_binding(&self) -> bool {
        // 1. Return true.
        true
    }

    /// 9.1.1.4.9 HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-hassuperbinding
    fn has_super_binding(&self) -> bool {
        // 1. Return false.
        false
    }

    /// 9.1.1.4.10 WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-withbaseobject
    fn with_base_object(&self) -> Option<ObjectAddr> {
        // 1. Return undefined.
        None
    }
}

impl GlobalEnvironment {
    /// 9.1.1.4.11 GetThisBinding ( )
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records-getthisbinding
    pub(crate) fn get_this_binding(&self) -> Option<ObjectAddr> {
        // 1. Return envRec.[[GlobalThisValue]].
        self.global_this_value.clone()
    }

    /// 9.1.1.4.12 HasLexicalDeclaration ( envRec, N )
    /// https://262.ecma-international.org/16.0/#sec-haslexicaldeclaration
    pub(crate) fn has_lexical_declaration(&self, name: &JSString) -> bool {
        // 1. Let DclRec be envRec.[[DeclarativeRecord]].
        let dcl_rec = &self.declarative_record;

        // 2. Return ! DclRec.HasBinding(N).
        dcl_rec.has_binding(name).unwrap_or(false)
    }

    /// 9.1.1.4.13 HasRestrictedGlobalProperty ( envRec, N )
    /// https://262.ecma-international.org/16.0/#sec-hasrestrictedglobalproperty
    pub(crate) fn has_restricted_global_property(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_rec = &self.object_record;

        // 2. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_rec.binding_object.clone();

        // 3. Let existingProp be ? globalObject.[[GetOwnProperty]](N).
        let existing_prop_opt = global_object.get_own_property(&JSObjectPropKey::from(name))?;

        // 4. If existingProp is undefined, return false.
        let Some(existing_prop) = existing_prop_opt else {
            return Ok(false);
        };

        // 5. If existingProp.[[Configurable]] is true, return false.
        if existing_prop.configurable == Some(true) {
            return Ok(false);
        }

        // 6. Return true.
        Ok(true)
    }

    /// 9.1.1.4.14 CanDeclareGlobalVar ( envRec, N )
    /// https://262.ecma-international.org/16.0/#sec-candeclareglobalvar
    pub(crate) fn can_declare_global_var(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_rec = &self.object_record;

        // 2. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_rec.binding_object.clone();

        // 3. Let existingProp be ? globalObject.[[GetOwnProperty]](N).
        let existing_prop_opt = global_object.get_own_property(&JSObjectPropKey::from(name))?;

        // 4. If existingProp is undefined, return true.
        let Some(existing_prop) = existing_prop_opt else {
            return Ok(true);
        };

        // 5. If existingProp.[[Configurable]] is true, return true.
        if existing_prop.configurable == Some(true) {
            return Ok(true);
        }

        // 6. Return false.
        Ok(false)
    }

    /// 9.1.1.4.15 CanDeclareGlobalFunction ( envRec, N )
    /// https://262.ecma-international.org/16.0/#sec-candeclareglobalfunction
    pub(crate) fn can_declare_global_function(&self, name: &JSString) -> CompletionRecord<bool> {
        // 1. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_rec = &self.object_record;

        // 2. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_rec.binding_object.clone();

        // 3. Let existingProp be ? globalObject.[[GetOwnProperty]](N).
        let existing_prop_opt = global_object.get_own_property(&JSObjectPropKey::from(name))?;

        // 4. If existingProp is undefined, return ? IsExtensible(globalObject).
        let Some(existing_prop) = existing_prop_opt else {
            return Ok(is_extensible(&global_object));
        };

        // 5. If existingProp.[[Configurable]] is true, return true.
        if existing_prop.configurable == Some(true) {
            return Ok(true);
        }

        // 6. If IsDataDescriptor(existingProp) is true and existingProp has attribute values { [[Writable]]: true, [[Enumerable]]: true }, return true.
        if existing_prop.is_data_descriptor()
            && existing_prop.writable == Some(true)
            && existing_prop.enumerable == Some(true)
        {
            return Ok(true);
        }

        // 7. Return false.
        Ok(false)
    }

    /// 9.1.1.4.16 CreateGlobalVarBinding ( envRec, N, D )
    /// https://262.ecma-international.org/16.0/#sec-createglobalvarbinding
    pub(crate) fn create_global_var_binding(
        &mut self,
        name: JSString,
        deletable: bool,
    ) -> CompletionRecord {
        // 1. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_rec = &mut self.object_record;

        // 2. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_rec.binding_object.clone();

        // 3. Let hasProperty be ? HasOwnProperty(globalObject, N).
        let has_property = has_own_property(&global_object, &JSObjectPropKey::from(&name))?;

        // 4. Let extensible be ? IsExtensible(globalObject).
        let extensible = is_extensible(&global_object);

        // 5. If hasProperty is false and extensible is true, then
        if !has_property && extensible {
            // a. Perform ? ObjRec.CreateMutableBinding(N, D).
            obj_rec.create_mutable_binding(name.clone(), deletable)?;

            // b. Perform ? ObjRec.InitializeBinding(N, undefined).
            obj_rec.initialize_binding(name, JSValue::Undefined)?;
        }

        // 6. Return unused.
        Ok(())
    }

    /// 9.1.1.4.17 CreateGlobalFunctionBinding ( envRec, N, V, D )
    /// https://262.ecma-international.org/16.0/#sec-createglobalfunctionbinding
    pub(crate) fn create_global_function_binding(
        &mut self,
        name: JSString,
        value: JSValue,
        deletable: bool,
    ) -> CompletionRecord {
        // 1. Let ObjRec be envRec.[[ObjectRecord]].
        let obj_rec = &self.object_record;

        // 2. Let globalObject be ObjRec.[[BindingObject]].
        let global_object = obj_rec.binding_object.clone();

        // 3. Let existingProp be ? globalObject.[[GetOwnProperty]](N).
        let existing_prop_opt = global_object.get_own_property(&JSObjectPropKey::from(&name))?;

        // 4. If existingProp is undefined or existingProp.[[Configurable]] is true, then
        let desc = if existing_prop_opt.is_none()
            || existing_prop_opt
                .is_some_and(|existing_prop| existing_prop.configurable == Some(true))
        {
            // a. Let desc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: D }.
            JSObjectPropDescriptor {
                value: Some(value.clone()),
                writable: Some(true),
                enumerable: Some(true),
                configurable: Some(deletable),
                ..JSObjectPropDescriptor::default()
            }
        }
        // 5. Else,
        else {
            // a. Let desc be the PropertyDescriptor { [[Value]]: V }.
            JSObjectPropDescriptor {
                value: Some(value.clone()),
                ..JSObjectPropDescriptor::default()
            }
        };

        // 6. Perform ? DefinePropertyOrThrow(globalObject, N, desc).
        define_property_or_throw(&global_object, &JSObjectPropKey::from(&name), desc)?;

        // 7. Perform ? Set(globalObject, N, V, false).
        set(&global_object, &JSObjectPropKey::from(&name), value, false)?;

        // 8. Return unused.
        Ok(())
    }
}
