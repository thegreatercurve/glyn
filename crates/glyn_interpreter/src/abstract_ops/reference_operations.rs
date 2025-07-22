use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::EnvironmentMethods,
        reference::{Reference, ReferenceBase},
    },
    value::string::JSString,
    JSValue,
};

/// 6.2.5.2 IsUnresolvableReference ( V )
/// https://262.ecma-international.org/16.0/#sec-isunresolvablereference
fn is_unresolvable_reference(value: &Reference) -> bool {
    // 1. If V.[[Base]] is unresolvable, return true; otherwise return false.
    value.base == ReferenceBase::Unresolvable
}

/// 6.2.5.8 InitializeReferencedBinding ( V, W )
/// https://262.ecma-international.org/16.0/#sec-initializereferencedbinding
pub(crate) fn initialize_referenced_binding<'a>(
    reference: Reference,
    value: JSValue,
) -> CompletionRecord {
    // 1. Assert: IsUnresolvableReference(V) is false.
    debug_assert!(!is_unresolvable_reference(&reference));

    // 2. Let base be V.[[Base]].
    let base = reference.base;

    // 3. Assert: base is an Environment Record.
    debug_assert!(matches!(base, ReferenceBase::Environment(_)));

    let ReferenceBase::Environment(mut env_addr) = base else {
        unreachable!()
    };

    // 4. Return ? base.InitializeBinding(V.[[ReferencedName]], W).
    env_addr.initialize_binding(JSString::try_from(&reference.referenced_name)?, value)
}
