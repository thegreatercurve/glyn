use std::rc::Rc;

use crate::{
    runtime::{agent::JSAgent, completion::NormalCompletion, realm::Realm, script::ScriptRecord},
    value::JSValue,
};

/// https://github.com/tc39/test262/blob/main/INTERPRETING.md
pub fn eval_script(agent: &mut JSAgent, script_str: &str) -> Result<JSValue, String> {
    // 1. Let hostDefined be any host-defined values for the provided sourceText (obtained in an implementation dependent manner)
    let host_defined = None;

    // 2. Let realm be the current Realm Record.
    let realm = Rc::new(Realm::create_realm(agent));

    // 3. Let s be ParseScript(sourceText, realm, hostDefined).
    let s = ScriptRecord::default().parse_script(agent, script_str, realm, host_defined);

    // 4. If s is a List of errors, then
    // a. Let error be the first element of s.
    // b. Return Completion{[[Type]]: throw, [[Value]]: error, [[Target]]: empty}.
    // 5. Let status be ScriptEvaluation(s).
    let status = s.script_evaluation(agent);

    // 6. Return Completion(status).
    // NOTE: We only return JSValue to avoid needing to expose additional types.
    match status {
        Ok(NormalCompletion::Boolean(value)) => Ok(JSValue::Boolean(value)),
        Ok(NormalCompletion::Value(value)) => Ok(value),
        Ok(NormalCompletion::Unused) => Ok(JSValue::Undefined),
        Err(err) => Err(format!("Script parsing error: {err:?}")),
    }
}
