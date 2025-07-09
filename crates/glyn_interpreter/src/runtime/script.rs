use crate::{codegen::bytecode::generator::FinalProgram, runtime::realm::RealmAddr};

/// 16.1.4 Script Records
/// https://262.ecma-international.org/16.0/#script-record
#[derive(Clone, Debug)]
pub(crate) struct ScriptRecord {
    /// [[Realm]]
    pub(crate) realm: RealmAddr,

    /// [[ECMAScriptCode]]
    pub(crate) ecmascript_code: FinalProgram,

    /// [[HostDefined]]
    pub(crate) host_defined: Option<()>,
}
