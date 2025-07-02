use crate::{value::object::JSObjAddr, JSAgent};

/// 20.2.3 Properties of the Function Prototype Object
/// https://262.ecma-international.org/15.0/#sec-properties-of-the-function-prototype-object
#[derive(Debug)]
pub(crate) struct FunctionPrototype;

impl FunctionPrototype {
    pub(crate) fn create(agent: &mut JSAgent) -> JSObjAddr {
        // has an [[Extensible]] internal slot whose value is true.
        // has the internal methods defined for ordinary objects, except for the [[SetPrototypeOf]] method, which is as defined in 10.4.7.1. (Thus, it is an immutable prototype exotic object.)
        // has an internal slot named [[Prototype]] whose value is null.
        todo!()
    }
}
