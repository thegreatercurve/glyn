use crate::runtime::agent::JSAgent;
use crate::runtime::environment::Environment;
use crate::value::object::JSObjAddr;

/// 9.1.2.5 NewGlobalEnvironment ( G, thisValue )
/// https://262.ecma-international.org/15.0/#sec-newglobalenvironment
pub(crate) fn new_global_environment(
    _agent: &mut JSAgent,
    _global_object: JSObjAddr,
    _this_value: JSObjAddr,
) -> Environment {
    // TODO: Implement proper global environment creation
    Environment
}
