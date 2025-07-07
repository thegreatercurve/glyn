use std::rc::Rc;

use crate::{
    runtime::{
        agent::JSAgent,
        completion::CompletionRecord,
        execution_context::{ExecutionContext, ScriptOrModule},
        realm::Realm,
    },
    value::JSValue,
};

/// 16.1.4 Script Records
/// https://262.ecma-international.org/15.0/#script-record
#[derive(Debug)]
pub(crate) struct ScriptRecord {
    /// [[Realm]]
    pub(crate) realm: Rc<Realm>,

    /// [[HostDefined]]
    pub(crate) host_defined: Option<()>,
}
