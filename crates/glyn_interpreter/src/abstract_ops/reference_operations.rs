use crate::{
    runtime::{
        completion::CompletionRecord,
        reference::{Reference, ReferenceBase},
    },
    JSAgent, JSValue,
};

/// 6.2.5.2 IsUnresolvableReference ( V )
/// https://262.ecma-international.org/16.0/#sec-isunresolvablereference
fn is_unresolvable_reference(agent: &JSAgent, value: &Reference) -> bool {
    // 1. If V.[[Base]] is unresolvable, return true; otherwise return false.
    value.base == ReferenceBase::Unresolvable
}

/// 6.2.5.8 InitializeReferencedBinding ( V, W )
/// https://262.ecma-international.org/16.0/#sec-initializereferencedbinding
pub(crate) fn initialize_referenced_binding(
    agent: &mut JSAgent,
    reference: Reference,
    value: JSValue,
) -> CompletionRecord {
    // 1. Assert: IsUnresolvableReference(V) is false.
    debug_assert!(!is_unresolvable_reference(agent, &reference));

    // 2. Let base be V.[[Base]].
    let base = reference.base;

    // 3. Assert: base is an Environment Record.
    debug_assert!(matches!(base, ReferenceBase::Environment(_)));

    let ReferenceBase::Environment(env_addr) = base else {
        unreachable!()
    };

    // 4. Return ? base.InitializeBinding(V.[[ReferencedName]], W).
    (agent.environment(env_addr).methods.initialize_binding)(
        agent,
        env_addr,
        reference.referenced_name.as_string().unwrap(),
        value,
    )
}
