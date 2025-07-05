use crate::{
    runtime::{environment::Environment, realm::Realm, script::ScriptRecord},
    value::object::JSObjAddr,
};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum ScriptOrModule {
    Script(Rc<ScriptRecord>),
    Module,
}

/// 9.4 Execution Contexts
/// https://262.ecma-international.org/15.0/#sec-execution-contexts
#[derive(Debug, Default)]
pub struct ExecutionContext {
    /// Function
    pub function: Option<JSObjAddr>,

    // Realm
    pub realm: Rc<Realm>,

    /// ScriptOrModule
    pub script_or_module: Option<ScriptOrModule>,

    /// LexicalEnvironment
    pub lexical_environment: Option<Rc<Environment>>,

    /// VariableEnvironment
    pub variable_environment: Option<Rc<Environment>>,

    /// PrivateEnvironment
    pub private_environment: Option<Rc<Environment>>,
}
