use std::rc::Rc;

use crate::runtime::environment::Environment;
use crate::runtime::realm::Realm;
use crate::runtime::script::ScriptRecord;
use crate::value::object::JSObjAddr;

#[derive(Debug)]
pub(crate) enum ScriptOrModule {
    Script(Rc<ScriptRecord>),
    Module,
}

/// 9.4 Execution Contexts
/// https://262.ecma-international.org/15.0/#sec-execution-contexts
#[derive(Debug, Default)]
pub(crate) struct ExecutionContext {
    /// Function
    pub(crate) function: Option<JSObjAddr>,

    // Realm
    pub(crate) realm: Rc<Realm>,

    /// ScriptOrModule
    pub(crate) script_or_module: Option<ScriptOrModule>,

    /// LexicalEnvironment
    pub(crate) lexical_environment: Option<Rc<Environment>>,

    /// VariableEnvironment
    pub(crate) variable_environment: Option<Rc<Environment>>,

    /// PrivateEnvironment
    pub(crate) private_environment: Option<Rc<Environment>>,
}
