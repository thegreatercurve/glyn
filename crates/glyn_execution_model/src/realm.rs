use std::rc::Rc;

use crate::{environment::Environment, intrinsics::Intrinsics, value::object::JSObjAddr};

/// 9.3 Realms
/// https://262.ecma-international.org/15.0/#sec-code-realms
#[derive(Debug, Default)]
pub struct Realm {
    /// [[Intrinsics]]
    pub intrinsics: Intrinsics,

    /// [[GlobalObject]]
    pub global_object: Option<JSObjAddr>,

    /// [[GlobalEnv]]
    pub global_env: Option<Rc<Environment>>,
}
