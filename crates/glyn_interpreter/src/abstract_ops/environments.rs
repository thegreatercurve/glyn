use crate::runtime::agent::JSAgent;
use crate::runtime::completion::CompletionRecord;
use crate::runtime::environment::{Environment, EnvironmentAddr, EnvironmentKind};
use crate::runtime::reference::{Reference, ReferenceBase, ReferenceName};
use crate::value::object::JSObjAddr;
use crate::value::string::JSString;

/// 9.1.2.1 GetIdentifierReference ( env, name, strict )
/// https://262.ecma-international.org/16.0/#sec-getidentifierreference
pub(crate) fn get_identifier_reference(
    agent: &JSAgent,
    env: Option<EnvironmentAddr>,
    name: JSString,
    strict: bool,
) -> CompletionRecord<Reference> {
    // 1. If env is null, then
    let Some(env) = env else {
        // a. Return the Reference Record { [[Base]]: unresolvable, [[ReferencedName]]: name, [[Strict]]: strict, [[ThisValue]]: empty }.
        return Ok(Reference {
            base: ReferenceBase::Unresolvable,
            referenced_name: ReferenceName::from(name),
            strict,
            this_value: None,
        });
    };

    // 2. Let exists be ? env.HasBinding(name).
    let exists = (agent.environment(env).methods.has_binding)(agent, env, &name)?;

    // 3. If exists is true, then
    if exists {
        // a. Return the Reference Record { [[Base]]: env, [[ReferencedName]]: name, [[Strict]]: strict, [[ThisValue]]: empty }.
        return Ok(Reference {
            base: ReferenceBase::Environment(env),
            referenced_name: ReferenceName::from(name),
            strict,
            this_value: None,
        });
    }

    // 4. Else,
    // a. Let outer be env.[[OuterEnv]].
    let outer = agent.environment(env).outer;

    // b. Return ? GetIdentifierReference(outer, name, strict).
    get_identifier_reference(agent, outer, name, strict)
}

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
