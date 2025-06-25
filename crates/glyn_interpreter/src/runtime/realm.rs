use std::any::Any;

use crate::value::JSObject;

#[derive(Debug)]
struct Environment;

#[derive(Debug, Default)]
struct Intrinsics;

/// 9.3 Realms
/// https://262.ecma-international.org/15.0/#sec-code-realms
#[derive(Debug, Default)]
pub struct Realm {
    /// [[Intrinsics]]
    intrinsics: Intrinsics,

    /// [[GlobalObject]]
    global_object: Option<Box<JSObject>>,

    /// [[GlobalEnv]]
    global_env: Option<Box<Environment>>,

    /// [[HostDefined]]
    host_defined: Option<Box<dyn Any>>,
}

impl Realm {
    /// 9.3.1 CreateRealm ( )
    /// https://262.ecma-international.org/15.0/#sec-createrealm
    pub fn create_realm(&mut self) -> Self {
        // 1. Let realmRec be a new Realm Record.
        let realm_rec = Realm::default();

        // 2. Perform CreateIntrinsics(realmRec).
        self.create_intrinsics();

        // 3. Set realmRec.[[AgentSignifier]] to AgentSignifier().
        // 4. Set realmRec.[[GlobalObject]] to undefined.
        // 5. Set realmRec.[[GlobalEnv]] to undefined.
        // 6. Set realmRec.[[TemplateMap]] to a new empty List.
        // 7. Return realmRec.

        realm_rec
    }

    /// 9.3.2 CreateIntrinsics ( realmRec )
    /// https://262.ecma-international.org/15.0/#sec-createintrinsics
    pub fn create_intrinsics(&mut self) {
        // 1. Set realmRec.[[Intrinsics]] to a new Record.
        self.intrinsics = Intrinsics::default();
        // 2. Set fields of realmRec.[[Intrinsics]] with the values listed in Table 6. The field names are the names listed in column one of the table. The value of each field is a new object value fully and recursively populated with property values as defined by the specification of each object in clauses 19 through 28. All object property values are newly created object values. All values that are built-in function objects are created by performing CreateBuiltinFunction(steps, length, name, slots, realmRec, prototype) where steps is the definition of that function provided by this specification, name is the initial value of the function's "name" property, length is the initial value of the function's "length" property, slots is a list of the names, if any, of the function's specified internal slots, and prototype is the specified value of the function's [[Prototype]] internal slot. The creation of the intrinsics and their properties must be ordered to avoid any dependencies upon objects that have not yet been created.
        // 3. Perform AddRestrictedFunctionProperties(realmRec.[[Intrinsics]].[[%Function.prototype%]], realmRec).
        // 4. Return unused.
    }
}
