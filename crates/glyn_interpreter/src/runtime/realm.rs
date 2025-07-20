use crate::gc::Gc;
use crate::runtime::environment::EnvironmentAddr;
use crate::runtime::intrinsics::Intrinsics;
use crate::value::object::ObjectAddr;

pub(crate) type RealmAddr = Gc<Realm>;

/// 9.3 Realms
/// https://262.ecma-international.org/16.0/#sec-code-realms
#[derive(Debug, Default)]
pub(crate) struct Realm {
    /// [[Intrinsics]]
    pub(crate) intrinsics: Intrinsics,

    /// [[GlobalObject]]
    pub(crate) global_object: Option<ObjectAddr>,

    /// [[GlobalEnv]]
    pub(crate) global_env: Option<EnvironmentAddr>,
}
