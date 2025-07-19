use crate::{
    abstract_ops::environments::get_identifier_reference,
    runtime::{
        agent::JSAgent, completion::CompletionRecord, environment::EnvironmentAddr,
        execution_context::ScriptOrModule, reference::Reference,
    },
    value::string::JSString,
};

/// 9.4.1 GetActiveScriptOrModule ( )
/// https://262.ecma-international.org/16.0/#sec-getactivescriptormodule
pub(crate) fn get_active_script_or_module(agent: &JSAgent) -> Option<&ScriptOrModule> {
    // 1. If the execution context stack is empty, return null.
    if agent.execution_contexts.is_empty() {
        return None;
    }

    // 2. Let ec be the topmost execution context on the execution context stack whose ScriptOrModule component is not null.
    for execution_context in agent.execution_contexts.iter().rev() {
        if let Some(ref script_or_module) = execution_context.script_or_module {
            // 3. If no such execution context exists, return null. Otherwise, return ec's ScriptOrModule.
            return Some(script_or_module);
        }
    }

    None
}

/// 9.4.2 ResolveBinding ( name [ , env ] )
/// https://262.ecma-international.org/16.0/#sec-resolvebinding
pub(crate) fn resolve_binding(
    agent: &JSAgent,
    name: &JSString,
    env: Option<EnvironmentAddr>,
) -> CompletionRecord<Reference> {
    // 1. If env is not present or env is undefined, then
    let env = match env {
        Some(env) => env,
        // a. Set env to the running execution context's LexicalEnvironment.
        None => agent
            .running_execution_context()
            .lexical_environment
            .clone()
            .unwrap(),
    };

    // 2. Assert: env is an Environment Record.
    // 3. Let strict be IsStrict(the syntactic production that is being evaluated).
    // TODO: Grab the strict mode flag from the parser state.
    let strict = true;

    // 4. Return ? GetIdentifierReference(env, name, strict).
    get_identifier_reference(Some(env), name, strict)
}
