use crate::{
    abstract_ops::{
        realm::{create_realm, initialize_host_defined_realm},
        script::{parse_script, script_evaluation},
    },
    runtime::agent::JSAgent,
    value::JSValue,
};

/// https://github.com/tc39/test262/blob/main/INTERPRETING.md
pub fn eval_script(agent: &mut JSAgent, script_str: &str) -> Result<JSValue, String> {
    // 1. Let hostDefined be any host-defined values for the provided sourceText (obtained in an implementation dependent manner)
    let host_defined = None;

    // 2. Let realm be the current Realm Record.
    let _ = initialize_host_defined_realm(agent);
    let realm = create_realm(agent);

    // 3. Let s be ParseScript(sourceText, realm, hostDefined).
    let s = parse_script(agent, script_str, realm, host_defined);

    // 4. If s is a List of errors, then
    // a. Let error be the first element of s.
    // b. Return Completion{[[Type]]: throw, [[Value]]: error, [[Target]]: empty}.
    // 5. Let status be ScriptEvaluation(s).
    let status = script_evaluation(agent, &s);

    // 6. Return Completion(status).
    // NOTE: We only return JSValue to avoid needing to expose additional types.
    match status {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Script parsing error: {err:?}")),
    }
}
