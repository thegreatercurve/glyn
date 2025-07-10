use crate::runtime::agent::JSAgent;
use crate::runtime::environment::{Environment, EnvironmentAddr, EnvironmentKind};
use crate::value::object::JSObjAddr;

/// 9.1.2.5 NewGlobalEnvironment ( G, thisValue )
/// https://262.ecma-international.org/16.0/#sec-newglobalenvironment
pub(crate) fn new_global_environment(
    agent: &mut JSAgent,
    _global_object: JSObjAddr,
    _this_value: JSObjAddr,
) -> EnvironmentAddr {
    // TODO: Implement proper global environment creation
    agent.allocate_environment(Environment::new(EnvironmentKind::Global))
}
