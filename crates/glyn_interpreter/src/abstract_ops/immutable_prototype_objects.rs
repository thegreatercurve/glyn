use crate::{value::object::JSObjAddr, JSAgent};

/// 10.4.7.2 SetImmutablePrototype ( O, V )
/// https://262.ecma-international.org/15.0/#sec-set-immutable-prototype
pub(crate) fn set_immutable_prototype(
    agent: &mut JSAgent,
    obj_addr: JSObjAddr,
    value_addr: Option<JSObjAddr>,
) -> bool {
    // 1. Let current be ? O.[[GetPrototypeOf]]().
    let opt_current_addr = (agent.object(obj_addr).methods.get_prototype_of)(agent, obj_addr);

    // 2. If SameValue(V, current) is true, return true.
    if let (Some(value), Some(current)) = (value_addr, opt_current_addr) {
        if current == value {
            return true;
        }
    }

    // 3. Return false.
    false
}
