use crate::runtime::{agent::JSAgent, execution_context::ScriptOrModule};

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
