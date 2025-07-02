use crate::{
    runtime::{
        agent::{ExecutionContext, JSAgent},
        completion::{CompletionRecord, NormalCompletion},
        realm::Realm,
    },
    value::JSValue,
};

/// 16.1.4 Script Records
/// https://262.ecma-international.org/15.0/#script-record
#[derive(Debug, Default)]
pub(crate) struct ScriptRecord {
    /// [[Realm]]
    pub realm: Box<Realm>,

    /// [[HostDefined]]
    pub host_defined: Option<()>,
}

impl ScriptRecord {
    /// 16.1.5 ParseScript ( sourceText, realm, hostDefined )
    /// https://262.ecma-international.org/15.0/#sec-parse-script
    pub(crate) fn parse_script(
        &self,
        _agent: &mut JSAgent,
        _source_text: &str,
        realm: Box<Realm>,
        host_defined: Option<()>,
    ) -> Self {
        // 1. Let script be ParseText(sourceText, Script)
        // 2. If script is a List of errors, return script.
        // TODO Implement parse text and error handling.

        // 3. Return Script Record { [[Realm]]: realm, [[ECMAScriptCode]]: script, [[LoadedModules]]: « », [[HostDefined]]: hostDefined }.
        Self {
            realm,
            host_defined,
        }
    }

    /// 16.1.6 ScriptEvaluation ( scriptRecord )
    /// https://262.ecma-international.org/15.0/#sec-runtime-semantics-scriptevaluation
    pub(crate) fn script_evaluation(&self, agent: &mut JSAgent) -> CompletionRecord {
        // 1. Let globalEnv be scriptRecord.[[Realm]].[[GlobalEnv]].
        let _global_env = &self.realm.global_env;

        // 2. Let scriptContext be a new ECMAScript code execution context.
        let script_context = ExecutionContext {
            realm: self.realm.clone(),
        };

        // 3. Set the Function of scriptContext to null.
        // 4. Set the Realm of scriptContext to scriptRecord.[[Realm]].
        // 5. Set the ScriptOrModule of scriptContext to scriptRecord.
        // 6. Set the VariableEnvironment of scriptContext to globalEnv.
        // 7. Set the LexicalEnvironment of scriptContext to globalEnv.
        // 8. Set the PrivateEnvironment of scriptContext to null.
        // 9. Suspend the running execution context.
        // 10. Push scriptContext onto the execution context stack; scriptContext is now the running execution context.
        agent.push_execution_context(script_context);

        // 11. Let script be scriptRecord.[[ECMAScriptCode]].
        // 12. Let result be Completion(GlobalDeclarationInstantiation(script, globalEnv)).
        // 13. If result is a normal completion, then
        // a. Set result to Completion(Evaluation of script).
        // b. If result is a normal completion and result.[[Value]] is empty, then
        // i. Set result to NormalCompletion(undefined).
        let result = Ok(NormalCompletion::Value(JSValue::Undefined));

        // 14. Suspend scriptContext and remove it from the execution context stack.
        // 15. Assert: The execution context stack is not empty.
        // 16. Resume the context that is now on the top of the execution context stack as the running execution context.

        // 17. Return ? result.
        result
    }
}
