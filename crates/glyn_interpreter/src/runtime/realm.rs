use safe_gc::Trace;

use crate::{
    intrinsics::{function_prototype::FunctionPrototype, object_prototype::JSObjectPrototype},
    runtime::{environment::Environment, intrinsics::Intrinsics},
    value::object::JSObjAddr,
    JSAgent,
};

/// 9.3 Realms
/// https://262.ecma-international.org/15.0/#sec-code-realms
#[derive(Debug)]
pub struct Realm {
    /// [[Intrinsics]]
    pub intrinsics: Intrinsics,

    /// [[GlobalObject]]
    global_object: Option<Box<JSObjAddr>>,

    /// [[GlobalEnv]]
    pub global_env: Option<Box<Environment>>,
}

impl Trace for Realm {
    fn trace(&self, _collector: &mut safe_gc::Collector) {}
}

impl Realm {
    /// 9.3.1 CreateRealm ( )
    /// https://262.ecma-international.org/15.0/#sec-createrealm
    pub fn create_realm(agent: &mut JSAgent) -> Self {
        // 1. Let realmRec be a new Realm Record.
        let realm_rec = Realm {
            // 2. Perform CreateIntrinsics(realmRec).
            intrinsics: Self::create_intrinsics(agent),

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
}
