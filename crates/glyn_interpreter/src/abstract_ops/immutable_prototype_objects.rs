use crate::value::object::{ObjectAddr, ObjectEssentialInternalMethods};

/// 10.4.7.2 SetImmutablePrototype ( O, V )
/// https://262.ecma-international.org/16.0/#sec-set-immutable-prototype
pub(crate) fn set_immutable_prototype(
    object: &impl ObjectEssentialInternalMethods,
    value_addr: Option<ObjectAddr>,
) -> bool {
    // 1. Let current be ? O.[[GetPrototypeOf]]().
    let opt_current_addr = object.get_prototype_of();

    // 2. If SameValue(V, current) is true, return true.
    if let (Some(value), Some(current)) = (value_addr, opt_current_addr) {
        if current == value {
            return true;
        }
    }

    // 3. Return false.
    false
}
