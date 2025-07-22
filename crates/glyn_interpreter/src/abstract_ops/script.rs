use crate::{
    codegen::{bytecode::generator::ExecutableProgram, parser::Parser},
    lexer::Lexer,
    runtime::{
        agent::{syntax_error, JSAgent},
        completion::CompletionRecord,
        environment::{EnvironmentAddr, EnvironmentMethods},
        execution_context::{ExecutionContext, ScriptOrModule},
        realm::RealmAddr,
        script::ScriptRecord,
    },
    value::{string::JSString, JSValue},
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
    let global_env = script_record.realm.borrow_mut().global_env.clone();

    // 2. Let scriptContext be a new ECMAScript code execution context.
    let script_context = ExecutionContext {
        // 3. Set the Function of scriptContext to null.
        function: None,

        // 4. Set the Realm of scriptContext to scriptRecord.[[Realm]].
        realm: script_record.realm.clone(),

        // 5. Set the ScriptOrModule of scriptContext to scriptRecord.
        script_or_module: Some(ScriptOrModule::Script(script_record.clone())),

        // 6. Set the VariableEnvironment of scriptContext to globalEnv.
        variable_environment: global_env.clone(),

        // 7. Set the LexicalEnvironment of scriptContext to globalEnv.
        lexical_environment: global_env.clone(),

        // 8. Set the PrivateEnvironment of scriptContext to null.
        private_environment: None,
    };

    // 9. Suspend the running execution context.
    // 10. Push scriptContext onto the execution context stack; scriptContext is now the running execution context.
    agent.push_execution_context(script_context);

    // 11. Let script be scriptRecord.[[ECMAScriptCode]].
    let script = &script_record.ecmascript_code;

    // 12. Let result be Completion(GlobalDeclarationInstantiation(script, globalEnv)).
    global_declaration_instantiation(script, global_env)?;

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

/// 16.1.7 GlobalDeclarationInstantiation ( script, env )
/// https://262.ecma-international.org/16.0/#sec-globaldeclarationinstantiation
pub(crate) fn global_declaration_instantiation(
    script: &ExecutableProgram,
    env_opt: Option<EnvironmentAddr>,
) -> CompletionRecord {
    let env = env_opt.unwrap_or_else(|| unreachable!());

    // TODO: These are not correct and will require refinement.
    // 1. Let lexNames be the LexicallyDeclaredNames of script.
    let lex_names = script
        .identifiers
        .iter()
        .filter(|ident| ident.is_lexical_declaration())
        .collect::<Vec<_>>();

    // 2. Let varNames be the VarDeclaredNames of script.
    let _var_names = script
        .identifiers
        .iter()
        .filter(|ident| ident.is_variable_declaration())
        .collect::<Vec<_>>();

    // 3. For each element name of lexNames, do
    for name in &lex_names {
        // a. If HasLexicalDeclaration(env, name) is true, throw a SyntaxError exception.
        if env
            .borrow_mut()
            .as_global_mut()
            .unwrap_or_else(|| unreachable!())
            .has_lexical_declaration(&JSString::from(name.to_owned()))
        {
            syntax_error("Lexical declaration already exists on the global environment.");
        }

        // b. Let hasRestrictedGlobal be ? HasRestrictedGlobalProperty(env, name).
        // c. NOTE: Global var and function bindings (except those that are introduced by non-strict direct eval) are non-configurable and are therefore restricted global properties.
        // d. If hasRestrictedGlobal is true, throw a SyntaxError exception.
    }

    // 4. For each element name of varNames, do
    // a. If HasLexicalDeclaration(env, name) is true, throw a SyntaxError exception.
    // 5. Let varDeclarations be the VarScopedDeclarations of script.
    // 6. Let functionsToInitialize be a new empty List.
    // 7. Let declaredFunctionNames be a new empty List.
    // 8. For each element d of varDeclarations, in reverse List order, do
    // a. If d is not either a VariableDeclaration, a ForBinding, or a BindingIdentifier, then
    // i. Assert: d is either a FunctionDeclaration, a GeneratorDeclaration, an AsyncFunctionDeclaration, or an AsyncGeneratorDeclaration.
    // ii. NOTE: If there are multiple function declarations for the same name, the last declaration is used.
    // iii. Let fn be the sole element of the BoundNames of d.
    // iv. If declaredFunctionNames does not contain fn, then
    // 1. Let fnDefinable be ? CanDeclareGlobalFunction(env, fn).
    // 2. If fnDefinable is false, throw a TypeError exception.
    // 3. Append fn to declaredFunctionNames.
    // 4. Insert d as the first element of functionsToInitialize.
    // 9. Let declaredVarNames be a new empty List.
    // 10. For each element d of varDeclarations, do
    // a. If d is either a VariableDeclaration, a ForBinding, or a BindingIdentifier, then
    // i. For each String vn of the BoundNames of d, do
    // 1. If declaredFunctionNames does not contain vn, then
    // a. Let vnDefinable be ? CanDeclareGlobalVar(env, vn).
    // b. If vnDefinable is false, throw a TypeError exception.
    // c. If declaredVarNames does not contain vn, then
    // i. Append vn to declaredVarNames.
    // 11. NOTE: No abnormal terminations occur after this algorithm step if the global object is an ordinary object. However, if the global object is a Proxy exotic object it may exhibit behaviours that cause abnormal terminations in some of the following steps.
    // 12. NOTE: Annex B.3.2.2 adds additional steps at this point.
    // 13. Let lexDeclarations be the LexicallyScopedDeclarations of script.
    // 14. Let privateEnv be null.
    // 15. For each element d of lexDeclarations, do
    for d in &lex_names {
        // a. NOTE: Lexically declared names are only instantiated here but not initialized.
        // TODO: This is incorrect and will require refinement.

        // b. For each element dn of the BoundNames of d, do
        if d.is_lexical_declaration() {
            // i. If IsConstantDeclaration of d is true, then
            if d.is_constant_declaration() {
                // 1. Perform ? env.CreateImmutableBinding(dn, true).
                env.borrow_mut()
                    .as_global_mut()
                    .unwrap_or_else(|| unreachable!())
                    .create_immutable_binding(JSString::from(d.to_owned()), true)?;
            }
            // ii. Else,
            else {
                // 1. Perform ? env.CreateMutableBinding(dn, false).
                env.borrow_mut()
                    .as_global_mut()
                    .unwrap_or_else(|| unreachable!())
                    .create_mutable_binding(JSString::from(d.to_owned()), false)?;
            }
        }
    }

    // 16. For each Parse Node f of functionsToInitialize, do
    // a. Let fn be the sole element of the BoundNames of f.
    // b. Let fo be InstantiateFunctionObject of f with arguments env and privateEnv.
    // c. Perform ? CreateGlobalFunctionBinding(env, fn, fo, false).
    // 17. For each String vn of declaredVarNames, do
    // a. Perform ? CreateGlobalVarBinding(env, vn, false).

    // 18. Return unused.
    Ok(())
}
