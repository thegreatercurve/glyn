use std::rc::Rc;

use crate::runtime::environment::Environment;
use crate::runtime::intrinsics::Intrinsics;
use crate::value::object::JSObjAddr;

/// 9.3 Realms
/// https://262.ecma-international.org/15.0/#sec-code-realms
#[derive(Debug, Default)]
pub(crate) struct Realm {
    /// [[Intrinsics]]
    pub(crate) intrinsics: Intrinsics,

    /// [[GlobalObject]]
    pub(crate) global_object: Option<JSObjAddr>,

    /// [[GlobalEnv]]
    pub(crate) global_env: Option<Rc<Environment>>,
}
