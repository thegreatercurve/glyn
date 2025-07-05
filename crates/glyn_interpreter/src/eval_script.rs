use std::rc::Rc;

use crate::{
    abstract_ops::realm::create_realm,
    runtime::{agent::JSAgent, script::ScriptRecord},
    value::JSValue,
};

/// https://github.com/tc39/test262/blob/main/INTERPRETING.md
pub fn eval_script(agent: &mut JSAgent, script_str: &str) -> Result<JSValue, String> {
    // 1. Let hostDefined be any host-defined values for the provided sourceText (obtained in an implementation dependent manner)
    let host_defined = None;

    let realm = create_realm(agent);

    // 2. Let realm be the current Realm Record.
    let realm = Rc::new(realm);

    // 3. Let s be ParseScript(sourceText, realm, hostDefined).
    let s = ScriptRecord::parse_script(agent, script_str, realm, host_defined);

    // 4. If s is a List of errors, then
    // a. Let error be the first element of s.
    // b. Return Completion{[[Type]]: throw, [[Value]]: error, [[Target]]: empty}.
    // 5. Let status be ScriptEvaluation(s).
    let status = ScriptRecord::script_evaluation(agent, Rc::new(s));

    // 6. Return Completion(status).
    // NOTE: We only return JSValue to avoid needing to expose additional types.
    match status {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Script parsing error: {err:?}")),
    }
}
