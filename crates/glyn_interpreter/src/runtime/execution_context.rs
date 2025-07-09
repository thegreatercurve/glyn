use crate::runtime::environment::EnvironmentAddr;
use crate::runtime::realm::RealmAddr;
use crate::runtime::script::ScriptRecord;
use crate::value::object::JSObjAddr;

#[derive(Debug)]
pub(crate) enum ScriptOrModule {
    Script(ScriptRecord),
    Module,
}

/// 9.4 Execution Contexts
/// https://262.ecma-international.org/16.0/#sec-execution-contexts
#[derive(Debug)]
pub(crate) struct ExecutionContext {
    /// Function
    pub(crate) function: Option<JSObjAddr>,

    // Realm
    pub(crate) realm: RealmAddr,

    /// ScriptOrModule
    pub(crate) script_or_module: Option<ScriptOrModule>,

    /// LexicalEnvironment
    pub(crate) lexical_environment: Option<EnvironmentAddr>,

    /// VariableEnvironment
    pub(crate) variable_environment: Option<EnvironmentAddr>,

    /// PrivateEnvironment
    pub(crate) private_environment: Option<EnvironmentAddr>,
}
