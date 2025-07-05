use std::rc::Rc;

use crate::{
    abstract_ops::{environments::new_global_environment, object::ordinary_object_create},
    intrinsics::{function_prototype::FunctionPrototype, object_prototype::JSObjectPrototype},
    runtime::{intrinsics::Intrinsics, realm::Realm},
    value::object::JSObjAddr,
    JSAgent,
};

/// 9.3.1 CreateRealm ( )
/// https://262.ecma-international.org/15.0/#sec-createrealm
pub fn create_realm(agent: &mut JSAgent) -> Realm {
    // 1. Let realmRec be a new Realm Record.
    let realm_rec = Realm {
        // 2. Perform CreateIntrinsics(realmRec).
        intrinsics: create_intrinsics(agent),

        // 4. Set realmRec.[[GlobalObject]] to undefined.
        global_object: None,

        // 5. Set realmRec.[[GlobalEnv]] to undefined.
        global_env: None,
    };

    // 7. Return realmRec.
    realm_rec
}

/// 9.3.2 CreateIntrinsics ( realmRec )
/// https://262.ecma-international.org/15.0/#sec-createintrinsics
pub fn create_intrinsics(agent: &mut JSAgent) -> Intrinsics {
    // 1. Set realmRec.[[Intrinsics]] to a new Record.
    let mut intrinsics = Intrinsics::default();

    // Iniitalize the base object prototype first so it can be used in other intrinsics.
    intrinsics.object_prototype = Some(JSObjectPrototype::create(agent));

    // 2. Set fields of realmRec.[[Intrinsics]] with the values listed in Table 6. The field names are the names listed in column one of the table. The value of each field is a new object value fully and recursively populated with property values as defined by the specification of each object in clauses 19 through 28. All object property values are newly created object values. All values that are built-in function objects are created by performing CreateBuiltinFunction(steps, length, name, slots, realmRec, prototype) where steps is the definition of that function provided by this specification, name is the initial value of the function's "name" property, length is the initial value of the function's "length" property, slots is a list of the names, if any, of the function's specified internal slots, and prototype is the specified value of the function's [[Prototype]] internal slot. The creation of the intrinsics and their properties must be ordered to avoid any dependencies upon objects that have not yet been created.
    intrinsics.function_prototype = Some(FunctionPrototype::create(agent));

    // 3. Perform AddRestrictedFunctionProperties(realmRec.[[Intrinsics]].[[%Function.prototype%]], realmRec).
    // 4. Return unused.

    intrinsics
}

/// 9.3.3 SetRealmGlobalObject ( realmRec, globalObj, thisValue )
/// https://262.ecma-international.org/15.0/#sec-setrealmglobalobject
pub fn set_realm_global_object(
    agent: &mut JSAgent,
    mut realm_record: Realm,
    opt_global_obj_addr: Option<JSObjAddr>,
    this_value: Option<JSObjAddr>,
) {
    // 1. If globalObj is undefined, then
    let global_obj_addr = opt_global_obj_addr.unwrap_or_else(|| {
        // a. Let intrinsics be realmRec.[[Intrinsics]].
        let intrinsics = &realm_record.intrinsics;

        // b. Set globalObj to OrdinaryObjectCreate(intrinsics.[[%Object.prototype%]]).
        ordinary_object_create(agent, intrinsics.object_prototype, None)
    });

    // 2. Assert: globalObj is an Object.

    // 3. If thisValue is undefined, set thisValue to globalObj.
    let this_value = this_value.unwrap_or(global_obj_addr);

    // 4. Set realmRec.[[GlobalObject]] to globalObj.
    realm_record.global_object = Some(global_obj_addr);

    // 5. Let newGlobalEnv be NewGlobalEnvironment(globalObj, thisValue).
    let new_global_env = new_global_environment(agent, global_obj_addr, this_value);

    // 6. Set realmRec.[[GlobalEnv]] to newGlobalEnv.
    realm_record.global_env = Some(Rc::new(new_global_env));

    // 7. Return unused.
}
