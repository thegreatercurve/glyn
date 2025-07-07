use std::rc::Rc;

use crate::{codegen::bytecode::generator::FinalProgram, runtime::realm::Realm};

/// 16.1.4 Script Records
/// https://262.ecma-international.org/15.0/#script-record
#[derive(Debug)]
pub(crate) struct ScriptRecord {
    /// [[Realm]]
    pub(crate) realm: Rc<Realm>,

    /// [[ECMAScriptCode]]
    pub(crate) ecmascript_code: FinalProgram,

    /// [[HostDefined]]
    pub(crate) host_defined: Option<()>,
}
