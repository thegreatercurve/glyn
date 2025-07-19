use crate::{
    abstract_ops::function_operations::create_builtin_function,
    runtime::{agent::JSAgent, realm::RealmAddr},
    value::{
        object::{property::JSObjectPropKey, JSObjAddr},
        JSValue,
    },
};

/// 20.2.3 Properties of the Function Prototype Object
/// https://262.ecma-international.org/16.0/#sec-properties-of-the-function-prototype-object
#[derive(Debug)]
pub(crate) struct FunctionPrototype;

impl FunctionPrototype {
    pub(crate) fn create(agent: &mut JSAgent, realm_addr: RealmAddr) -> JSObjAddr {
        // accepts any arguments and returns undefined when invoked.
        let behaviour_fn = |_agent: &mut JSAgent, _args: Vec<JSValue>| JSValue::Undefined;

        // is itself a built-in function object.
        create_builtin_function(
            agent,
            behaviour_fn,
            // has a "length" property whose value is +0𝔽.
            0,
            // has a "name" property whose value is the empty String.
            JSObjectPropKey::String("".into()),
            // does not have a [[Construct]] internal method; it cannot be used as a constructor with the new operator.
            vec![],
            Some(realm_addr),
            // has a [[Prototype]] internal slot whose value is %Object.prototype%.
            agent
                .heap
                .realm(realm_addr)
                .intrinsics
                .object_prototype,
            None,
        )
    }
}
