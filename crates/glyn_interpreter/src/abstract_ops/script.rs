use crate::{
    codegen::{bytecode::generator::ExecutableProgram, parser::Parser},
    lexer::Lexer,
    runtime::{
        agent::JSAgent,
        completion::CompletionRecord,
        execution_context::{ExecutionContext, ScriptOrModule},
        realm::RealmAddr,
        script::ScriptRecord,
    },
    value::JSValue,
    vm::VM,
};

/// 11.1.6 Static Semantics: ParseText ( sourceText, goalSymbol )
/// https://262.ecma-international.org/16.0/#sec-parsetext
pub(crate) fn parse_text(source_text: &str) -> Result<ExecutableProgram, String> {
    // 1. Attempt to parse sourceText using goalSymbol as the goal symbol, and analyse the parse result for any early error conditions. Parsing and early error detection may be interleaved in an implementation-defined manner.
    let lexer = Lexer::new(source_text);
    let mut parser = Parser::new(lexer);

    // 2. If the parse succeeded and no early errors were found, return the Parse Node (an instance of goalSymbol) at the root of the parse tree resulting from the parse.
    parser.js_parse_script().map_err(|e| e.to_string())?;
    Ok(parser.program())

    // 3. Otherwise, return a List of one or more SyntaxError objects representing the parsing errors and/or early errors. If more than one parsing error or early error is present, the number and ordering of error objects in the list is implementation-defined, but at least one must be present.
}

/// 16.1.5 ParseScript ( sourceText, realm, hostDefined )
/// https://262.ecma-international.org/16.0/#sec-parse-script
pub(crate) fn parse_script(
    _agent: &mut JSAgent,
    source_text: &str,
    realm_addr: RealmAddr,
    host_defined: Option<()>,
) -> Result<ScriptRecord, String> {
    // 1. Let script be ParseText(sourceText, Script)
    // 2. If script is a List of errors, return script.
    let script = parse_text(source_text)?;

    // 3. Return Script Record { [[Realm]]: realm, [[ECMAScriptCode]]: script, [[LoadedModules]]: « », [[HostDefined]]: hostDefined }.
    Ok(ScriptRecord {
        realm: realm_addr,
        ecmascript_code: script,
        host_defined,
    })
}

/// 16.1.6 ScriptEvaluation ( scriptRecord )
/// https://262.ecma-international.org/16.0/#sec-runtime-semantics-scriptevaluation
pub(crate) fn script_evaluation(
    agent: &mut JSAgent,
    script_record: &ScriptRecord,
) -> CompletionRecord<JSValue> {
    // 1. Let globalEnv be scriptRecord.[[Realm]].[[GlobalEnv]].
    let global_env = &agent.realm(script_record.realm).global_env;

    // 2. Let scriptContext be a new ECMAScript code execution context.
    let script_context = ExecutionContext {
        // 3. Set the Function of scriptContext to null.
        function: None,

        // 4. Set the Realm of scriptContext to scriptRecord.[[Realm]].
        realm: script_record.realm,

        // 5. Set the ScriptOrModule of scriptContext to scriptRecord.
        script_or_module: Some(ScriptOrModule::Script(script_record.clone())),

        // 6. Set the VariableEnvironment of scriptContext to globalEnv.
        variable_environment: *global_env,

        // 7. Set the LexicalEnvironment of scriptContext to globalEnv.
        lexical_environment: *global_env,

        // 8. Set the PrivateEnvironment of scriptContext to null.
        private_environment: None,
    };

    // 9. Suspend the running execution context.
    // 10. Push scriptContext onto the execution context stack; scriptContext is now the running execution context.
    agent.push_execution_context(script_context);

    // 11. Let script be scriptRecord.[[ECMAScriptCode]].
    let script = &script_record.ecmascript_code;

    // 12. Let result be Completion(GlobalDeclarationInstantiation(script, globalEnv)).
    // 13. If result is a normal completion, then
    // a. Set result to Completion(Evaluation of script).
    let opt_result = VM::new(agent, script).evaluate_script();

    // b. If result is a normal completion and result.[[Value]] is empty, then
    let Ok(result) = opt_result else {
        // i. Set result to NormalCompletion(undefined).
        return Ok(JSValue::Undefined);
    };

    // 14. Suspend scriptContext and remove it from the execution context stack.
    // 15. Assert: The execution context stack is not empty.
    // 16. Resume the context that is now on the top of the execution context stack as the running execution context.

    // 17. Return ? result.
    Ok(result)
}
